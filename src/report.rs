// src/report.rs
use crate::engine::RouteOption;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub fn generate_pdf_report(routes: &[RouteOption], output_path: &str) {
    println!("📄 Generating PDF Report at {}...", output_path);

    let (doc, page1, layer1) = PdfDocument::new("A.L.I.S. Engineering Report", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Load a standard font
    let font = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
    let font_reg = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();

    // Title
    current_layer.use_text("PROJECT A.L.I.S. - Technical Evaluation Report", 18.0, Mm(20.0), Mm(270.0), &font);
    
    let mut y_pos = 250.0;

    for route in routes {
        let yield_pct = crate::six_sigma::calculate_yield(route.estimated_defects, route.total_opportunities);
        let score = route.calculate_lean_score();

        current_layer.use_text(format!("Route: {}", route.route_name), 14.0, Mm(20.0), Mm(y_pos), &font);
        y_pos -= 7.0;
        current_layer.use_text(format!("  Lean Score: {:.2} | Process Yield: {:.2}%", score, yield_pct), 12.0, Mm(25.0), Mm(y_pos), &font_reg);
        
        y_pos -= 15.0; // Space for next entry

        if y_pos < 30.0 { break; } // Simple page overflow protection
    }

    doc.save(&mut BufWriter::new(File::create(output_path).unwrap())).unwrap();
    println!("✅ PDF Report saved successfully!");
}