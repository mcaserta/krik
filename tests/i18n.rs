use krik::I18nManager;

#[test]
fn test_translation_system() {
    // Test English (default)
    assert_eq!(
        I18nManager::translate_string("document_information", "en"),
        "Document Information"
    );
    assert_eq!(
        I18nManager::translate_string("document_downloaded_from", "en"),
        "This document was downloaded from"
    );
    assert_eq!(
        I18nManager::translate_string("generated_at", "en"),
        "Generated at"
    );

    // Test Italian
    assert_eq!(
        I18nManager::translate_string("document_information", "it"),
        "Informazioni sul Documento"
    );
    assert_eq!(
        I18nManager::translate_string("document_downloaded_from", "it"),
        "Questo documento è stato scaricato da"
    );
    assert_eq!(
        I18nManager::translate_string("generated_at", "it"),
        "Generato il"
    );

    // Test Spanish
    assert_eq!(
        I18nManager::translate_string("document_information", "es"),
        "Información del Documento"
    );
    assert_eq!(
        I18nManager::translate_string("generated_at", "es"),
        "Generado el"
    );

    // Test unknown language defaults to English
    assert_eq!(
        I18nManager::translate_string("document_information", "unknown"),
        "Document Information"
    );
}