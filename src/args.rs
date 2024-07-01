use clap::Parser;

use problem::*;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "Progetto per l'esame di Elementi di Intelligenza Artificiale, realizzato da Adriano Oliviero (N46006115)."
)]
pub struct Args {
    /// File contenente il dataset
    #[arg(short = 'F', long = "file", default_value = "data/email-Enron.txt.gz")]
    pub file: Option<String>,

    /// Stato iniziale
    #[arg(short = 'i', long = "inizia", default_value = "46")]
    pub stato_iniziale: Option<u32>,

    /// Stato finale
    #[arg(short = 'f', long = "finale", default_value = "73")]
    pub stato_finale: Option<u32>,

    // Esegue tutti gli algoritmi di ricerca
    #[arg(
        short = 'a',
        long = "all",
        action,
        conflicts_with = "ricerca",
        default_value = "false"
    )]
    pub all: bool,

    #[arg(short = 'l', long="limite", default_value = "10")]
    pub limite: usize,

    /// Esegue un algoritmo di ricerca specifico
    #[arg(
        short = 'r',
        long = "ricerca",
        conflicts_with = "all",
        default_value = "bi-directional"
    )]
    pub ricerca: Option<Ricerca>,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            file: Some("email-Enron.txt.gz".into()),
            stato_iniziale: Some(46),
            stato_finale: Some(73),
            all: true,
            limite: 10,
            ricerca: Some(Ricerca::BiDirectional),
        }
    }
}
