use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    pub tarih: String,
    pub gunluk_test: String,
    pub gunluk_vaka: String,
    pub gunluk_hasta: String,
    pub gunluk_vefat: String,
    pub gunluk_iyilesen: String,
    pub toplam_test: String,
    pub toplam_hasta: String,
    pub toplam_vefat: String,
    pub toplam_iyilesen: String,
    pub toplam_yogun_bakim: String,
    pub toplam_entube: String,
    pub hastalarda_zaturre_oran: String,
    pub agir_hasta_sayisi: String,
    pub yatak_doluluk_orani: String,
    pub eriskin_yogun_bakim_doluluk_orani: String,
    pub ventilator_doluluk_orani: String,
    pub ortalama_filyasyon_suresi: String,
    pub ortalama_temasli_tespit_suresi: String,
    pub filyasyon_orani: String,
}

pub async fn covid() -> Result<Response, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://covid19.saglik.gov.tr")
        .await?
        .text()
        .await?;

    let baslangic = resp.find(r"var sondurumjson = [").unwrap();
    let bitis = resp.find("];//]]>").unwrap();
    let jsonstr = &resp[baslangic..bitis][20..];
    let json: Response = serde_json::from_str(jsonstr)?;
    Ok(json)
}
