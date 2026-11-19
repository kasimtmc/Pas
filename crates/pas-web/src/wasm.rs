//! WebAssembly arayüzü.
//!
//! JavaScript girdiyi (UTF-8 JSON) `bellek_ayir` ile ayrılan yere yazar ve
//! bir `pas_…` işlevini çağırır. İşlev sonucu (UTF-8 JSON) saklar ve
//! uzunluğunu döndürür; sonuç `sonuc_adresi()` adresinden okunur.

use std::cell::RefCell;

use serde_json::Value;

thread_local! {
    static SONUC: RefCell<String> = const { RefCell::new(String::new()) };
}

#[unsafe(no_mangle)]
pub extern "C" fn bellek_ayir(uzunluk: usize) -> *mut u8 {
    let mut alan = Vec::<u8>::with_capacity(uzunluk);
    let adres = alan.as_mut_ptr();
    std::mem::forget(alan);
    adres
}

/// # Safety
/// `adres`, `bellek_ayir(uzunluk)` ile ayrılmış olmalı.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bellek_birak(adres: *mut u8, uzunluk: usize) {
    drop(unsafe { Vec::from_raw_parts(adres, 0, uzunluk) });
}

#[unsafe(no_mangle)]
pub extern "C" fn sonuc_adresi() -> *const u8 {
    SONUC.with_borrow(|sonuc| sonuc.as_ptr())
}

/// # Safety
/// `adres`ten başlayan `uzunluk` bayt okunabilir olmalı.
unsafe fn cagir(adres: *const u8, uzunluk: usize, islem: impl FnOnce(&Value) -> Value) -> usize {
    let girdi = unsafe { std::slice::from_raw_parts(adres, uzunluk) };
    let girdi: Value = serde_json::from_slice(girdi).unwrap_or(Value::Null);
    let sonuc = islem(&girdi).to_string();
    let uzunluk = sonuc.len();
    SONUC.set(sonuc);
    uzunluk
}

fn metin<'a>(girdi: &'a Value, alan: &str) -> &'a str {
    girdi[alan].as_str().unwrap_or("")
}

fn sayi(girdi: &Value, alan: &str) -> u64 {
    girdi[alan].as_u64().unwrap_or(1)
}

/// `{kaynak, girdi, tohum}` → oyun alanına gönderilecek Rust kodu
///
/// # Safety
/// Bkz. [`cagir`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pas_derle(adres: *const u8, uzunluk: usize) -> usize {
    unsafe {
        cagir(adres, uzunluk, |g| crate::derlenecek_kod(metin(g, "kaynak"), metin(g, "girdi"), sayi(g, "tohum")).into())
    }
}

/// `{kaynak, girdi, tohum, hata_ciktisi}` → tanılamalar ve panik (bkz. [`crate::sonucu_coz`])
///
/// # Safety
/// Bkz. [`cagir`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pas_coz(adres: *const u8, uzunluk: usize) -> usize {
    unsafe {
        cagir(adres, uzunluk, |g| {
            crate::sonucu_coz(metin(g, "kaynak"), metin(g, "girdi"), sayi(g, "tohum"), metin(g, "hata_ciktisi"))
        })
    }
}

/// `{kaynak}` → sade Rust çevirisi
///
/// # Safety
/// Bkz. [`cagir`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pas_rust(adres: *const u8, uzunluk: usize) -> usize {
    unsafe { cagir(adres, uzunluk, |g| crate::sade_rust(metin(g, "kaynak")).into()) }
}

/// `{kaynak}` → renklendirme sözcükleri
///
/// # Safety
/// Bkz. [`cagir`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pas_renklendir(adres: *const u8, uzunluk: usize) -> usize {
    unsafe { cagir(adres, uzunluk, |g| crate::renklendir(metin(g, "kaynak"))) }
}

/// Sözlük
///
/// # Safety
/// Bkz. [`cagir`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pas_sozluk(adres: *const u8, uzunluk: usize) -> usize {
    unsafe { cagir(adres, uzunluk, |_| crate::sozluk_json()) }
}
