# covidtr
Verileri covid19.saglik.gov.tr'den Çeken Küçük Bir Crate

## Örnek
```
use covidtr::*;
#[tokio::main]
async fn main() {
  let resp = covid().await.unwrap();
  println!("{}", resp[0].tarih);
}
```
## Kullanabileceğiniz Tüm Veriler
https://docs.rs/covidtr/0.1.0/covidtr/struct.ActualJson.html
