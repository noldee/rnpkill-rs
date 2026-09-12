//! Self-update: descarga la última release de GitHub y reemplaza
//! el binario actual en ejecución.

use anyhow::{Context, Result};
use self_update::cargo_crate_version;
use self_update::Status;
pub fn run() -> Result<()> {
    println!("🔄 Buscando actualizaciones...");

    let status = self_update::backends::github::Update::configure()
        .repo_owner("noldee")
        .repo_name("rnpkill-rs")
        .bin_name("rnpkill-rs")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()
        .context("No se pudo configurar el actualizador")?
        .update()
        .context("No se pudo revisar/aplicar la actualización")?;

    match status {
        Status::UpToDate(v) => println!("✔️  Ya tienes la última versión ({v})"),
        Status::Updated(v) => println!("✅ Actualizado a la versión {v}"),
    }

    Ok(())
}
