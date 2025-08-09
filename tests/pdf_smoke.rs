use krik::generator::pdf::PdfGenerator;

#[test]
fn pdf_availability_check_does_not_panic() {
    // This test does not require pandoc/typst to be present; it only exercises the availability check.
    let _ = PdfGenerator::is_available();
}


