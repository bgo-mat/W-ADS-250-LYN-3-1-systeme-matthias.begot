use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

// Structures pour désérialiser la réponse
// (à adapter selon ce que vous voulez récupérer)
#[derive(Debug, Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChoice {
    pub text: Option<String>, // Pour l'ancien endpoint /v1/completions
    // ou
    // pub message: Option<OpenAIChatMessage>, // Pour /v1/chat/completions
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChatMessage {
    pub role: String,
    pub content: String,
}

// Structure pour le corps de la requête (exemple)
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIRequest {
    prompt: String,
}

pub async fn call_openai_handler(payload: web::Json<OpenAIRequest>) -> impl Responder {
    // Récupérer la clé OpenAI
    let api_key = std::env::var("OPENAI_API_KEY")
        .unwrap_or_else(|_| "CLE_OPENAI_ICI".to_string());

    // Corps JSON de la requête : ici on illustre l'endpoint /v1/completions
    // Pour l’endpoint /v1/chat/completions, il faut adapter (model, messages, etc.)
    let request_body = json!({
        "model": "text-davinci-003",
        "prompt": payload.prompt,
        "max_tokens": 50
    });

    let client = reqwest::Client::new();

    // Envoi de la requête
    let response = client
        .post("https://api.openai.com/v1/completions")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await;

    // Gestion d’erreurs éventuelles
    let response = match response {
        Ok(res) => {
            if !res.status().is_success() {
                let status = res.status();
                let error_text = res.text().await.unwrap_or_default();
                return HttpResponse::BadRequest().body(format!(
                    "Erreur API OpenAI (HTTP {}): {}",
                    status, error_text
                ));
            }
            res
        }
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!(
                "Impossible de contacter l’API OpenAI: {}",
                e
            ));
        }
    };

    // Désérialisation de la réponse
    let openai_response: OpenAIResponse = match response.json().await {
        Ok(json) => json,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!(
                "Impossible de désérialiser la réponse OpenAI: {}",
                e
            ));
        }
    };

    // Ici, on suppose qu'on récupère le texte depuis le premier choix
    if let Some(choice) = openai_response.choices.first() {
        // Selon l’endpoint utilisé, adaptez l’accès : choice.text ou choice.message.content
        if let Some(completion_text) = &choice.text {
            HttpResponse::Ok().json(json!({ "completion": completion_text }))
        } else {
            HttpResponse::Ok().json(json!({ "completion": "Aucun texte retourné" }))
        }
    } else {
        HttpResponse::Ok().json(json!({ "completion": "Aucun choix dans la réponse" }))
    }
}
