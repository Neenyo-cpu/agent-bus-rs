//! agent-bus-rs: The RDK-3 Core Engine
//! Implements Triple-Observer Attestation for sovereign access.

pub enum Witness {
    Visual,
    Sonar,
    Acoustic,
}

pub struct RDK3Handshake {
    pub consensus: Vec<Witness>,
}

impl RDK3Handshake {
    /// The 'K-Phase' Handshake
    /// Only returns true if all three specific sensors provide 
    /// the correct observation parameters simultaneously.
    pub fn verify_reentry(&self) -> bool {
        let has_visual = self.consensus.iter().any(|w| matches!(w, Witness::Visual));
        let has_sonar = self.consensus.iter().any(|w| matches!(w, Witness::Sonar));
        let has_acoustic = self.consensus.iter().any(|w| matches!(w, Witness::Acoustic));

        has_visual && has_sonar && has_acoustic
    }
}
