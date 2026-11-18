//! LSP mesajlarının çerçevelenmesi: `Content-Length` başlığı ve JSON gövde.

use std::io::{BufRead, Write};

use serde_json::Value;

/// Bir mesaj okur. Akış bittiyse `None`; gövde geçersiz JSON ise `Value::Null`.
pub fn oku(okuyucu: &mut impl BufRead) -> Option<Value> {
    let mut uzunluk = None;
    loop {
        let mut satir = String::new();
        if okuyucu.read_line(&mut satir).ok()? == 0 {
            return None;
        }
        let satir = satir.trim_end();
        if satir.is_empty() {
            break;
        }
        if let Some(deger) = satir.strip_prefix("Content-Length:") {
            uzunluk = deger.trim().parse::<usize>().ok();
        }
    }
    let mut govde = vec![0; uzunluk?];
    okuyucu.read_exact(&mut govde).ok()?;
    Some(serde_json::from_slice(&govde).unwrap_or(Value::Null))
}

pub fn yaz(yazici: &mut impl Write, mesaj: &Value) -> std::io::Result<()> {
    let govde = serde_json::to_vec(mesaj)?;
    write!(yazici, "Content-Length: {}\r\n\r\n", govde.len())?;
    yazici.write_all(&govde)?;
    yazici.flush()
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn gidip_donme() {
        let mut tampon = Vec::new();
        let mesaj = serde_json::json!({ "jsonrpc": "2.0", "method": "deneme", "params": { "ad": "şekil" } });
        yaz(&mut tampon, &mesaj).unwrap();
        yaz(&mut tampon, &mesaj).unwrap();
        let mut okuyucu = tampon.as_slice();
        assert_eq!(oku(&mut okuyucu), Some(mesaj.clone()));
        assert_eq!(oku(&mut okuyucu), Some(mesaj));
        assert_eq!(oku(&mut okuyucu), None);
    }
}
