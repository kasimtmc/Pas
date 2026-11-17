//! Pas kaynağındaki konumlar ile üretilen Rust kodundaki konumlar
//! arasındaki eşleme.
//!
//! Çevirici yalnızca tek tek sözcükleri değiştirir ve satır eklemez; bu
//! yüzden eşleme, değişen her parça için bir [`Segment`] tutmaktan ibarettir.
//! İki parça arasındaki her şey birebir aynıdır.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Segment {
    pub pas_start: usize,
    pub pas_len: usize,
    pub rust_start: usize,
    pub rust_len: usize,
}

/// Bir konum değişen bir parçanın ortasına düşerse nereye yuvarlanacağı.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bias {
    /// Bir aralığın başlangıcı: parçanın başına.
    Start,
    /// Bir aralığın sonu: parçanın sonuna.
    End,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceMap {
    segments: Vec<Segment>,
}

impl SourceMap {
    /// Parçalar kaynakta sıralı ve çakışmasız eklenmelidir.
    pub(crate) fn push(&mut self, segment: Segment) {
        debug_assert!(self.segments.last().is_none_or(|last| {
            last.pas_start + last.pas_len <= segment.pas_start && last.rust_start + last.rust_len <= segment.rust_start
        }));
        self.segments.push(segment);
    }

    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    pub fn rust_to_pas(&self, offset: usize, bias: Bias) -> usize {
        map(&self.segments, offset, bias, |s| (s.rust_start, s.rust_len), |s| (s.pas_start, s.pas_len))
    }

    pub fn pas_to_rust(&self, offset: usize, bias: Bias) -> usize {
        map(&self.segments, offset, bias, |s| (s.pas_start, s.pas_len), |s| (s.rust_start, s.rust_len))
    }

    /// Rust kodundaki bir aralığı Pas kaynağındaki karşılığına çevirir.
    pub fn rust_range_to_pas(&self, start: usize, end: usize) -> (usize, usize) {
        let start = self.rust_to_pas(start, Bias::Start);
        (start, self.rust_to_pas(end, Bias::End).max(start))
    }
}

fn map(
    segments: &[Segment],
    offset: usize,
    bias: Bias,
    from: impl Fn(&Segment) -> (usize, usize),
    to: impl Fn(&Segment) -> (usize, usize),
) -> usize {
    let idx = segments.partition_point(|s| from(s).0 <= offset);
    let Some(segment) = idx.checked_sub(1).map(|i| &segments[i]) else {
        return offset;
    };
    let (from_start, from_len) = from(segment);
    let (to_start, to_len) = to(segment);
    if offset == from_start {
        to_start
    } else if offset < from_start + from_len {
        match bias {
            Bias::Start => to_start,
            Bias::End => to_start + to_len,
        }
    } else {
        to_start + to_len + (offset - from_start - from_len)
    }
}

/// Bayt konumları ile (satır, sütun) çiftleri arasında dönüşüm yapar.
/// Satır ve sütunlar 1'den başlar; sütunlar karakter sayısıdır (rustc gibi).
pub struct LineIndex<'a> {
    text: &'a str,
    line_starts: Vec<usize>,
}

impl<'a> LineIndex<'a> {
    pub fn new(text: &'a str) -> Self {
        let line_starts = std::iter::once(0).chain(text.match_indices('\n').map(|(i, _)| i + 1)).collect();
        LineIndex { text, line_starts }
    }

    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    pub fn line_col(&self, offset: usize) -> (usize, usize) {
        let offset = floor_char_boundary(self.text, offset.min(self.text.len()));
        let line = self.line_starts.partition_point(|&s| s <= offset) - 1;
        let col = self.text[self.line_starts[line]..offset].chars().count();
        (line + 1, col + 1)
    }

    pub fn offset(&self, line: usize, col: usize) -> Option<usize> {
        let start = *self.line_starts.get(line.checked_sub(1)?)?;
        let text = self.line(line)?;
        let skip = col.checked_sub(1)?;
        match text.char_indices().nth(skip) {
            Some((i, _)) => Some(start + i),
            None => Some(start + text.len()),
        }
    }

    /// Satır sonu karakterleri olmadan bir satırın metni.
    pub fn line(&self, line: usize) -> Option<&'a str> {
        let start = *self.line_starts.get(line.checked_sub(1)?)?;
        let end = self.line_starts.get(line).map_or(self.text.len(), |&next| next - 1);
        Some(self.text[start..end].trim_end_matches('\r'))
    }

    pub fn line_start(&self, line: usize) -> Option<usize> {
        self.line_starts.get(line.checked_sub(1)?).copied()
    }
}

fn floor_char_boundary(text: &str, mut offset: usize) -> usize {
    while !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> SourceMap {
        // "yap x" (pas) -> "let x" (rust); "#[p] " eklemesi ve ardından "genel" -> "pub"
        let mut map = SourceMap::default();
        map.push(Segment { pas_start: 0, pas_len: 6, rust_start: 0, rust_len: 3 });
        map.push(Segment { pas_start: 10, pas_len: 0, rust_start: 7, rust_len: 5 });
        map.push(Segment { pas_start: 10, pas_len: 5, rust_start: 12, rust_len: 3 });
        map
    }

    #[test]
    fn maps_unchanged_text() {
        let map = sample();
        assert_eq!(map.rust_to_pas(4, Bias::Start), 7);
        assert_eq!(map.pas_to_rust(7, Bias::Start), 4);
        assert_eq!(map.rust_to_pas(16, Bias::Start), 16);
    }

    #[test]
    fn maps_changed_tokens() {
        let map = sample();
        assert_eq!(map.rust_range_to_pas(0, 3), (0, 6));
        assert_eq!(map.rust_to_pas(1, Bias::Start), 0);
        assert_eq!(map.rust_to_pas(1, Bias::End), 6);
        // Eklenen metin, kaynağında sıfır uzunluklu bir noktaya düşer.
        assert_eq!(map.rust_range_to_pas(7, 12), (10, 10));
        assert_eq!(map.rust_range_to_pas(12, 15), (10, 15));
    }

    #[test]
    fn line_index() {
        let text = "işlev ana() {\r\n    yazdır!(\"ş\");\n}";
        let idx = LineIndex::new(text);
        assert_eq!(idx.line_count(), 3);
        assert_eq!(idx.line(1), Some("işlev ana() {"));
        let off = text.find("yazdır").unwrap();
        assert_eq!(idx.line_col(off), (2, 5));
        assert_eq!(idx.offset(2, 5), Some(off));
        assert_eq!(idx.line_col(text.len()), (3, 2));
    }
}
