use std::process::{exit, Command};

fn main() {
    // Aktif PHP sürümünü al
    let active_version_output = Command::new("php")
        .arg("-v")
        .output()
        .expect("Komut çalıştırılamadı");

    if active_version_output.status.success() {
        let output_str = String::from_utf8_lossy(&active_version_output.stdout);
        let active_version = output_str.lines().next().unwrap_or("Bilinmeyen");

        println!("Mevcut aktif PHP sürümü: {}", active_version);
    } else {
        eprintln!("Aktif PHP sürümü alınamadı.");
    }

    println!("{}", "-".repeat(50));
    // Sistemde yüklü olan PHP sürümlerini al
    let list_versions_output = Command::new("update-alternatives")
        .arg("--list")
        .arg("php")
        .output()
        .expect("Komut çalıştırılamadı");

    if list_versions_output.status.success() {
        let output_str = String::from_utf8_lossy(&list_versions_output.stdout);
        let available_versions: Vec<&str> = output_str.lines().collect();

        if available_versions.is_empty() {
            println!("Sistemde yüklü PHP sürümü bulunamadı.");
            exit(1);
        }

        println!("Sistemde yüklü olan PHP sürümleri:");
        for (index, version) in available_versions.iter().enumerate() {
            println!("{}. {}", index + 1, version);
        }

        println!("{}", "-".repeat(50));
        println!("Çıkmak için listede olmayan bir sıra numarası girin.");

        println!("{}", "-".repeat(50));
        println!("Hangi sürümü etkinleştirmek istersiniz? (Sıra numarasını girin): ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Okuma hatası");
        let selected_index: usize = input.trim().parse().expect("Geçersiz index");

        if selected_index > 0 && selected_index <= available_versions.len() {
            let selected_version = available_versions[selected_index - 1];
            let set_php_version_command =
                format!("sudo update-alternatives --set php {}", selected_version);

            let set_version_output = Command::new("bash")
                .arg("-c")
                .arg(set_php_version_command)
                .output()
                .expect("Komut çalıştırılamadı");

            if set_version_output.status.success() {
                println!("PHP sürümü başarıyla değiştirildi: {}", selected_version);
            } else {
                eprintln!("PHP sürümü değiştirilirken bir hata oluştu.");
                exit(1);
            }
        } else {
            eprintln!("Geçersiz sıra numarası.");
            exit(1);
        }
    } else {
        eprintln!("PHP sürümleri listelenirken bir hata oluştu.");
        exit(1);
    }
}
