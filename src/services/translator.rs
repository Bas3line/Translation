use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationRequest {
    pub text: String,
    pub source_lang: String,
    pub target_lang: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResponse {
    pub translated_text: String,
    pub detected_language: Option<String>,
    pub confidence: Option<f32>,
}

#[async_trait]
pub trait TranslationProvider: Send + Sync {
    async fn translate(&self, request: &TranslationRequest) -> Result<TranslationResponse>;
    fn name(&self) -> &str;
    fn supports_language(&self, lang: &str) -> bool;
}

pub struct LibreTranslateProvider {
    base_url: String,
    client: reqwest::Client,
}

impl LibreTranslateProvider {
    pub fn new() -> Self {
        Self {
            base_url: "https://libretranslate.com".to_string(),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .user_agent("MegaChinese-Bot/1.0")
                .build()
                .unwrap(),
        }
    }

    #[allow(dead_code)]
    pub fn with_custom_instance(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .user_agent("MegaChinese-Bot/1.0")
                .build()
                .unwrap(),
        }
    }

    fn normalize_lang_code(&self, lang: &str) -> String {
        match lang.to_lowercase().as_str() {
            "zh" | "zh-cn" | "zh-hans" | "chinese" => "zh".to_string(),
            "zh-tw" | "zh-hant" => "zh".to_string(),
            "en" | "english" => "en".to_string(),
            "ja" | "japanese" => "ja".to_string(),
            "ko" | "korean" => "ko".to_string(),
            "de" | "german" => "de".to_string(),
            "fr" | "french" => "fr".to_string(),
            "es" | "spanish" => "es".to_string(),
            "it" | "italian" => "it".to_string(),
            "pt" | "portuguese" => "pt".to_string(),
            "ru" | "russian" => "ru".to_string(),
            "ar" | "arabic" => "ar".to_string(),
            "hi" | "hindi" => "hi".to_string(),
            other => other.to_string(),
        }
    }
}

#[derive(Deserialize)]
struct LibreTranslateResponse {
    #[serde(rename = "translatedText")]
    translated_text: String,
    #[serde(rename = "detectedLanguage")]
    detected_language: Option<DetectedLanguage>,
}

#[derive(Deserialize, Clone)]
struct DetectedLanguage {
    language: String,
    confidence: f32,
}

#[async_trait]
impl TranslationProvider for LibreTranslateProvider {
    async fn translate(&self, request: &TranslationRequest) -> Result<TranslationResponse> {
        let url = format!("{}/translate", self.base_url);

        let source_lang = self.normalize_lang_code(&request.source_lang);
        let target_lang = self.normalize_lang_code(&request.target_lang);

        let payload = serde_json::json!({
            "q": request.text,
            "source": source_lang,
            "target": target_lang,
            "format": "text"
        });

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        let api_response: LibreTranslateResponse = response.json().await?;

        let detected_lang = api_response.detected_language.as_ref().map(|d| d.language.clone());
        let confidence = api_response.detected_language.as_ref().map(|d| d.confidence);

        Ok(TranslationResponse {
            translated_text: api_response.translated_text,
            detected_language: detected_lang,
            confidence,
        })
    }

    fn name(&self) -> &str {
        "LibreTranslate"
    }

    fn supports_language(&self, _lang: &str) -> bool {
        true
    }
}

impl Default for LibreTranslateProvider {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MyMemoryProvider {
    client: reqwest::Client,
}

impl MyMemoryProvider {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .user_agent("MegaChinese-Bot/1.0")
                .build()
                .unwrap(),
        }
    }

    fn normalize_lang_code(&self, lang: &str) -> String {
        match lang.to_lowercase().as_str() {
            "zh" | "zh-cn" | "zh-hans" | "chinese" => "zh-CN".to_string(),
            "zh-tw" | "zh-hant" => "zh-TW".to_string(),
            "en" | "english" => "en-US".to_string(),
            "ja" | "japanese" => "ja-JP".to_string(),
            "ko" | "korean" => "ko-KR".to_string(),
            "de" | "german" => "de-DE".to_string(),
            "fr" | "french" => "fr-FR".to_string(),
            "es" | "spanish" => "es-ES".to_string(),
            "it" | "italian" => "it-IT".to_string(),
            "pt" | "portuguese" => "pt-PT".to_string(),
            "ru" | "russian" => "ru-RU".to_string(),
            other => format!("{}-{}", other, other.to_uppercase()),
        }
    }
}

#[derive(Deserialize)]
struct MyMemoryResponse {
    #[serde(rename = "responseData")]
    response_data: MyMemoryResponseData,
}

#[derive(Deserialize)]
struct MyMemoryResponseData {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

#[async_trait]
impl TranslationProvider for MyMemoryProvider {
    async fn translate(&self, request: &TranslationRequest) -> Result<TranslationResponse> {
        let source_lang = self.normalize_lang_code(&request.source_lang);
        let target_lang = self.normalize_lang_code(&request.target_lang);

        let url = format!(
            "https://api.mymemory.translated.net/get?q={}&langpair={}|{}",
            urlencoding::encode(&request.text),
            source_lang,
            target_lang
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;

        let api_response: MyMemoryResponse = response.json().await?;

        Ok(TranslationResponse {
            translated_text: api_response.response_data.translated_text,
            detected_language: None,
            confidence: None,
        })
    }

    fn name(&self) -> &str {
        "MyMemory"
    }

    fn supports_language(&self, _lang: &str) -> bool {
        true
    }
}

impl Default for MyMemoryProvider {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LingvaProvider {
    base_url: String,
    client: reqwest::Client,
}

impl LingvaProvider {
    pub fn new() -> Self {
        Self {
            base_url: "https://lingva.ml".to_string(),
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .user_agent("MegaChinese-Bot/1.0")
                .build()
                .unwrap(),
        }
    }

    fn normalize_lang_code(&self, lang: &str) -> String {
        match lang.to_lowercase().as_str() {
            "zh" | "zh-cn" | "zh-hans" | "chinese" => "zh".to_string(),
            "zh-tw" | "zh-hant" => "zh_HANT".to_string(),
            other => other.to_string(),
        }
    }
}

#[derive(Deserialize)]
struct LingvaResponse {
    translation: String,
}

#[async_trait]
impl TranslationProvider for LingvaProvider {
    async fn translate(&self, request: &TranslationRequest) -> Result<TranslationResponse> {
        let source_lang = self.normalize_lang_code(&request.source_lang);
        let target_lang = self.normalize_lang_code(&request.target_lang);

        let url = format!(
            "{}/api/v1/{}/{}/{}",
            self.base_url,
            source_lang,
            target_lang,
            urlencoding::encode(&request.text)
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;

        let api_response: LingvaResponse = response.json().await?;

        Ok(TranslationResponse {
            translated_text: api_response.translation,
            detected_language: None,
            confidence: None,
        })
    }

    fn name(&self) -> &str {
        "Lingva"
    }

    fn supports_language(&self, _lang: &str) -> bool {
        true
    }
}

impl Default for LingvaProvider {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TranslationService {
    providers: Vec<Arc<dyn TranslationProvider>>,
}

impl TranslationService {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    pub fn add_provider(&mut self, provider: Arc<dyn TranslationProvider>) {
        self.providers.push(provider);
    }

    pub async fn translate(&self, request: &TranslationRequest) -> Result<TranslationResponse> {
        if self.providers.is_empty() {
            return Err(anyhow!("No translation providers configured"));
        }

        let mut last_error = None;

        for provider in &self.providers {
            if provider.supports_language(&request.source_lang)
                && provider.supports_language(&request.target_lang)
            {
                match provider.translate(request).await {
                    Ok(response) => {
                        tracing::info!(
                            "Translation successful using provider: {}",
                            provider.name()
                        );
                        return Ok(response);
                    }
                    Err(e) => {
                        tracing::warn!(
                            "Provider {} failed: {}. Trying next provider...",
                            provider.name(),
                            e
                        );
                        last_error = Some(e);
                        continue;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("All translation providers failed")))
    }

    pub async fn translate_with_fallback(
        &self,
        text: &str,
        source_lang: &str,
        target_lang: &str,
    ) -> Result<String> {
        let request = TranslationRequest {
            text: text.to_string(),
            source_lang: source_lang.to_string(),
            target_lang: target_lang.to_string(),
        };

        let response = self.translate(&request).await?;
        Ok(response.translated_text)
    }
}

impl Default for TranslationService {
    fn default() -> Self {
        Self::new()
    }
}
