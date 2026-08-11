use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Spanish,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl Language {
    pub fn all() -> &'static [Language] {
        &[Language::English, Language::Spanish]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "English 🇬B",
            Language::Spanish => "Español 🇪S",
        }
    }
}

pub fn tr(lang: Language, key: &'static str) -> &'static str {
    match (lang, key) {
        // Core App & Navigation
        (Language::English, "app_title") => "Interrupt - Screen Break Enforcer",
        (Language::Spanish, "app_title") => "Interrupt - Control de Pausas de Pantalla",

        (Language::English, "play_heading") => "SYSTEM ACTIVE // PLAY MODE",
        (Language::Spanish, "play_heading") => "SISTEMA ACTIVO // MODO JUEGO",

        (Language::English, "warning_heading") => "⚠️ SCREEN BREAK IMMINENT",
        (Language::Spanish, "warning_heading") => "⚠️ DESCANSO DE PANTALLA INMINENTE",

        (Language::English, "lock_now") => "🔒 Lock Now",
        (Language::Spanish, "lock_now") => "🔒 Bloquear Ahora",

        (Language::English, "reset_timer") => "🔄 Reset Timer",
        (Language::Spanish, "reset_timer") => "🔄 Reiniciar Temporizador",

        (Language::English, "settings") => "⚙️ Settings",
        (Language::Spanish, "settings") => "⚙️ Configuración",

        (Language::English, "exit") => "❌ Exit",
        (Language::Spanish, "exit") => "❌ Salir",

        (Language::English, "password_prompt") => "Enter password:",
        (Language::Spanish, "password_prompt") => "Ingrese contraseña:",

        (Language::English, "password_placeholder") => "Type password here...",
        (Language::Spanish, "password_placeholder") => "Escriba la contraseña aquí...",

        (Language::English, "unlock") => "Unlock",
        (Language::Spanish, "unlock") => "Desbloquear",

        (Language::English, "confirm") => "Confirm",
        (Language::Spanish, "confirm") => "Confirmar",

        (Language::English, "cancel") => "Cancel",
        (Language::Spanish, "cancel") => "Cancelar",

        (Language::English, "close") => "Close",
        (Language::Spanish, "close") => "Cerrar",

        (Language::English, "save_settings") => "💾 Save Settings",
        (Language::Spanish, "save_settings") => "💾 Guardar Configuración",

        (Language::English, "settings_saved") => "✅ Settings saved successfully!",
        (Language::Spanish, "settings_saved") => "✅ ¡Configuración guardada con éxito!",

        (Language::English, "invalid_password") => "Incorrect password!",
        (Language::Spanish, "invalid_password") => "¡Contraseña incorrecta!",

        // Settings Tabs
        (Language::English, "tab_general") => "⚙️ General",
        (Language::Spanish, "tab_general") => "⚙️ General",

        (Language::English, "tab_screensaver") => "🎨 Screensaver",
        (Language::Spanish, "tab_screensaver") => "🎨 Salvapantallas",

        (Language::English, "tab_math") => "🧮 Math Lock",
        (Language::Spanish, "tab_math") => "🧮 Bloqueo Matemático",

        (Language::English, "tab_geography") => "🌍 Geography Lock",
        (Language::Spanish, "tab_geography") => "🌍 Bloqueo Geográfico",

        (Language::English, "tab_vocab") => "📚 Vocab & Spelling",
        (Language::Spanish, "tab_vocab") => "📚 Vocabulario y Ortografía",

        // General Settings Labels
        (Language::English, "language_label") => "Interface Language:",
        (Language::Spanish, "language_label") => "Idioma de la interfaz:",

        (Language::English, "play_duration_label") => "Play Time (minutes):",
        (Language::Spanish, "play_duration_label") => "Tiempo de Juego (minutos):",

        (Language::English, "pause_duration_label") => "Pause Break Time (minutes):",
        (Language::Spanish, "pause_duration_label") => "Tiempo de Pausa (minutos):",

        (Language::English, "warning_duration_label") => "Warning Alert Time (seconds):",
        (Language::Spanish, "warning_duration_label") => "Tiempo de Advertencia (segundos):",

        (Language::English, "change_password_label") => "Change Password (leave empty to keep current):",
        (Language::Spanish, "change_password_label") => "Cambiar Contraseña (dejar vacío para mantener la actual):",

        (Language::English, "screensaver_style_label") => "Active Screensaver Variant:",
        (Language::Spanish, "screensaver_style_label") => "Variante de Salvapantallas Activa:",

        // Exercise Settings Labels
        (Language::English, "questions_required_label") => "Questions Required to Unlock:",
        (Language::Spanish, "questions_required_label") => "Preguntas Requeridas para Desbloquear:",

        (Language::English, "min_pause_percent_label") => "Minimum Pause Time Required (% of total break):",
        (Language::Spanish, "min_pause_percent_label") => "Tiempo Mínimo de Pausa Requerido (% del descanso):",

        (Language::English, "difficulty_label") => "Difficulty Level / Grade Level:",
        (Language::Spanish, "difficulty_label") => "Nivel de Dificultad / Nivel Escolar:",

        // Screensavers Text
        (Language::English, "aurora_inhale") => "🫁 Inhale deeply...",
        (Language::Spanish, "aurora_inhale") => "🫁 Inhala profundamente...",

        (Language::English, "aurora_hold") => "⏸️ Hold...",
        (Language::Spanish, "aurora_hold") => "⏸️ Mantén el aire...",

        (Language::English, "aurora_exhale") => "😮‍💨 Exhale slowly...",
        (Language::Spanish, "aurora_exhale") => "😮‍💨 Exhala lentamente...",

        (Language::English, "aurora_heading") => "🌿 TIME TO TAKE A BREAK",
        (Language::Spanish, "aurora_heading") => "🌿 ES HORA DE TOMAR UN DESCANSO",

        (Language::English, "aurora_subtext") => "Step away, stretch, drink water, and rest your eyes.",
        (Language::Spanish, "aurora_subtext") => "Aléjate de la pantalla, estírate, bebe agua y descansa los ojos.",

        (Language::English, "minimalist_heading") => "PAUSE",
        (Language::Spanish, "minimalist_heading") => "PAUSA",

        (Language::English, "minimalist_subtext") => "Resting computer screen...",
        (Language::Spanish, "minimalist_subtext") => "Descansando la pantalla...",

        (Language::English, "matrix_heading") => "SYSTEM PAUSED // SCREEN BREAK",
        (Language::Spanish, "matrix_heading") => "SISTEMA EN PAUSA // DESCANSO",

        (Language::English, "matrix_subtext") => "> Stand up and stretch before returning to console.",
        (Language::Spanish, "matrix_subtext") => "> Levántate y estírate antes de volver a la consola.",

        (Language::English, "vocab_title") => "📚 Vocabulary & Spelling Quiz",
        (Language::Spanish, "vocab_title") => "📚 Quiz de Vocabulario y Ortografía",

        (Language::English, "correct_feedback") => "✨ Correct! Great job.",
        (Language::Spanish, "correct_feedback") => "✨ ¡Correcto! Buen trabajo.",

        (Language::English, "incorrect_feedback") => "❌ Incorrect. Try again!",
        (Language::Spanish, "incorrect_feedback") => "❌ Incorrecto. ¡Inténtalo de nuevo!",

        (Language::English, "min_time_remaining") => "Complete questions & wait for minimum timer to finish.",
        (Language::Spanish, "min_time_remaining") => "Completa las preguntas y espera a que finalice el tiempo mínimo.",

        // Reset Dialog
        (Language::English, "reset_dialog_title") => "Reset Timer Confirmation",
        (Language::Spanish, "reset_dialog_title") => "Confirmación de Reinicio de Temporizador",

        (Language::English, "reset_dialog_text") => "Enter password to reset the play timer back to zero:",
        (Language::Spanish, "reset_dialog_text") => "Ingrese la contraseña para reiniciar el temporizador de juego:",

        // Fallback
        (_, _) => key,
    }
}

pub struct VocabItem {
    pub prompt: &'static str,
    pub correct: &'static str,
    pub wrong: [&'static str; 3],
}

pub fn get_vocab_question_pool(lang: Language, difficulty: crate::config::VocabDifficulty) -> &'static [VocabItem] {
    use crate::config::VocabDifficulty::*;

    match (lang, difficulty) {
        (Language::English, Low) => &[
            VocabItem {
                prompt: "Unscramble the letters: A - P - P - L - E",
                correct: "Apple",
                wrong: ["Applied", "Appeal", "Paper"],
            },
            VocabItem {
                prompt: "Which letter completes the word: c _ t (a domestic feline)?",
                correct: "a (cat)",
                wrong: ["o (cot)", "u (cut)", "e (cet)"],
            },
            VocabItem {
                prompt: "What is a synonym for 'Happy'?",
                correct: "Joyful",
                wrong: ["Sad", "Angry", "Tired"],
            },
            VocabItem {
                prompt: "Which word is spelled correctly?",
                correct: "School",
                wrong: ["Skool", "Schoole", "Schol"],
            },
            VocabItem {
                prompt: "Unscramble the letters: B - O - O - K",
                correct: "Book",
                wrong: ["Bake", "Boot", "Block"],
            },
            VocabItem {
                prompt: "What is the opposite (antonym) of 'Big'?",
                correct: "Small",
                wrong: ["Huge", "Tall", "Heavy"],
            },
            VocabItem {
                prompt: "Which word means 'the color of the clear sky'?",
                correct: "Blue",
                wrong: ["Green", "Yellow", "Red"],
            },
            VocabItem {
                prompt: "Unscramble the letters: F - R - I - E - N - D",
                correct: "Friend",
                wrong: ["Finder", "Field", "Frame"],
            },
            VocabItem {
                prompt: "Which word is spelled correctly?",
                correct: "Garden",
                wrong: ["Garten", "Gardin", "Gardan"],
            },
            VocabItem {
                prompt: "What is a synonym for 'Quick'?",
                correct: "Fast",
                wrong: ["Slow", "Heavy", "Quiet"],
            },
        ],
        (Language::English, Medium) => &[
            VocabItem {
                prompt: "Unscramble the letters: B - E - A - U - T - I - F - U - L",
                correct: "Beautiful",
                wrong: ["Bountiful", "Beautifying", "Plentiful"],
            },
            VocabItem {
                prompt: "What is the antonym of 'Ancient'?",
                correct: "Modern",
                wrong: ["Old", "Historic", "Aged"],
            },
            VocabItem {
                prompt: "Which word means 'a large natural stream of water'?",
                correct: "River",
                wrong: ["Ocean", "Pond", "Puddle"],
            },
            VocabItem {
                prompt: "Identify the correctly spelled word:",
                correct: "Necessary",
                wrong: ["Necesary", "Neccessary", "Nessesary"],
            },
            VocabItem {
                prompt: "What is a synonym for 'Courageous'?",
                correct: "Brave",
                wrong: ["Timid", "Fearful", "Cautious"],
            },
            VocabItem {
                prompt: "Unscramble the letters: K - N - O - W - L - E - D - G - E",
                correct: "Knowledge",
                wrong: ["Knowing", "Knighted", "Kingdom"],
            },
            VocabItem {
                prompt: "Which word means 'an instrument used for measuring temperature'?",
                correct: "Thermometer",
                wrong: ["Barometer", "Speedometer", "Microscope"],
            },
            VocabItem {
                prompt: "What is the antonym of 'Generous'?",
                correct: "Stingy",
                wrong: ["Kind", "Helpful", "Giving"],
            },
            VocabItem {
                prompt: "Identify the correctly spelled word:",
                correct: "Environment",
                wrong: ["Enviroment", "Enviromint", "Enveronment"],
            },
            VocabItem {
                prompt: "Which word means 'to bring or come to an end'?",
                correct: "Terminate",
                wrong: ["Initiate", "Continue", "Extend"],
            },
        ],
        (Language::English, High) => &[
            VocabItem {
                prompt: "Which word means 'showing strong feelings or passion'?",
                correct: "Fervent",
                wrong: ["Apathetic", "Lethargic", "Indifferent"],
            },
            VocabItem {
                prompt: "Identify the correctly spelled word:",
                correct: "Occurrence",
                wrong: ["Occurence", "Ocurrence", "Ocurrens"],
            },
            VocabItem {
                prompt: "What is the synonym for 'Meticulous'?",
                correct: "Painstaking",
                wrong: ["Careless", "Hasty", "Sloppy"],
            },
            VocabItem {
                prompt: "Which word means 'existing or occurring at the same time'?",
                correct: "Simultaneous",
                wrong: ["Sequential", "Consecutive", "Sporadic"],
            },
            VocabItem {
                prompt: "Unscramble the letters: M - I - S - C - H - I - E - V - O - U - S",
                correct: "Mischievous",
                wrong: ["Mischevious", "Mischivous", "Mischief"],
            },
            VocabItem {
                prompt: "What is the antonym of 'Ephemeral'?",
                correct: "Permanent",
                wrong: ["Fleeting", "Transient", "Brief"],
            },
            VocabItem {
                prompt: "Identify the correctly spelled word:",
                correct: "Accommodate",
                wrong: ["Acommodate", "Accomodate", "Acomodate"],
            },
            VocabItem {
                prompt: "Which word means 'expressing opinions in a concise and forceful way'?",
                correct: "Pithy",
                wrong: ["Verbose", "Rambling", "Wordy"],
            },
            VocabItem {
                prompt: "What is a synonym for 'Ubiquitous'?",
                correct: "Omnipresent",
                wrong: ["Rare", "Scarce", "Isolated"],
            },
            VocabItem {
                prompt: "Unscramble the letters: P - E - R - S - E - V - E - R - A - N - C - E",
                correct: "Perseverance",
                wrong: ["Perseverence", "Preservance", "Persevering"],
            },
        ],
        (Language::Spanish, Low) => &[
            VocabItem {
                prompt: "Descifra las letras: M - A - N - Z - A - N - A",
                correct: "Manzana",
                wrong: ["Mañana", "Mandarina", "Manubrio"],
            },
            VocabItem {
                prompt: "¿Qué letra completa la palabra: p _ r r o (mascota canina)?",
                correct: "e (perro)",
                wrong: ["a (parro)", "o (porro)", "u (purro)"],
            },
            VocabItem {
                prompt: "¿Cuál es un sinónimo de 'Feliz'?",
                correct: "Alegre",
                wrong: ["Triste", "Enojado", "Cansado"],
            },
            VocabItem {
                prompt: "¿Cuál palabra está escrita correctamente?",
                correct: "Escuela",
                wrong: ["Escwela", "Eskuela", "Escuella"],
            },
            VocabItem {
                prompt: "Descifra las letras: L - I - B - R - O",
                correct: "Libro",
                wrong: ["Librero", "Lente", "Limpio"],
            },
            VocabItem {
                prompt: "¿Cuál es el antónimo de 'Grande'?",
                correct: "Pequeño",
                wrong: ["Enorme", "Alto", "Pesado"],
            },
            VocabItem {
                prompt: "¿Qué palabra describe el color del cielo despejado?",
                correct: "Azul",
                wrong: ["Verde", "Amarillo", "Rojo"],
            },
            VocabItem {
                prompt: "Descifra las letras: A - M - I - G - O",
                correct: "Amigo",
                wrong: ["Amargo", "Ancho", "Anillo"],
            },
            VocabItem {
                prompt: "¿Cuál palabra está escrita correctamente?",
                correct: "Jardín",
                wrong: ["Gardin", "Jardin", "Jardim"],
            },
            VocabItem {
                prompt: "¿Cuál es un sinónimo de 'Rápido'?",
                correct: "Veloz",
                wrong: ["Lento", "Pesado", "Silencioso"],
            },
        ],
        (Language::Spanish, Medium) => &[
            VocabItem {
                prompt: "Descifra las letras: E - S - T - R - E - L - L - A",
                correct: "Estrella",
                wrong: ["Escarabajo", "Estrena", "Estante"],
            },
            VocabItem {
                prompt: "¿Cuál es el antónimo de 'Antiguo'?",
                correct: "Moderno",
                wrong: ["Viejo", "Histórico", "Anciano"],
            },
            VocabItem {
                prompt: "¿Qué palabra significa 'gran corriente natural de agua'?",
                correct: "Río",
                wrong: ["Océano", "Charco", "Lago"],
            },
            VocabItem {
                prompt: "Identifica la palabra escrita correctamente:",
                correct: "Excepción",
                wrong: ["Esecepción", "Exepción", "Escepción"],
            },
            VocabItem {
                prompt: "¿Cuál es un sinónimo de 'Valiente'?",
                correct: "Corajudo",
                wrong: ["Miedoso", "Temeroso", "Prudente"],
            },
            VocabItem {
                prompt: "Descifra las letras: S - A - B - I - D - U - R - Í - A",
                correct: "Sabiduría",
                wrong: ["Saber", "Sabueso", "Salubridad"],
            },
            VocabItem {
                prompt: "¿Qué instrumento sirve para medir la temperatura?",
                correct: "Termómetro",
                wrong: ["Barómetro", "Velocímetro", "Microscopio"],
            },
            VocabItem {
                prompt: "¿Cuál es el antónimo de 'Generoso'?",
                correct: "Tacaño",
                wrong: ["Amable", "Solidario", "Atento"],
            },
            VocabItem {
                prompt: "Identifica la palabra escrita correctamente:",
                correct: "Medioambiente",
                wrong: ["Medioanbiente", "Medio ambiente", "Medioamviente"],
            },
            VocabItem {
                prompt: "¿Qué palabra significa 'dar fin a algo'?",
                correct: "Terminar",
                wrong: ["Iniciar", "Continuar", "Ampliar"],
            },
        ],
        (Language::Spanish, High) => &[
            VocabItem {
                prompt: "¿Qué palabra significa 'que muestra fuerte entusiasmo o pasión'?",
                correct: "Ferviente",
                wrong: ["Apático", "Letárgico", "Indiferente"],
            },
            VocabItem {
                prompt: "Identifica la palabra escrita correctamente:",
                correct: "Idiosincrasia",
                wrong: ["Idiosincracia", "Idiosincrazia", "Idiosincraxia"],
            },
            VocabItem {
                prompt: "¿Cuál es un sinónimo de 'Meticuloso'?",
                correct: "Minucioso",
                wrong: ["Descuidado", "Apresurado", "Impreciso"],
            },
            VocabItem {
                prompt: "¿Qué palabra significa 'que ocurre al mismo tiempo'?",
                correct: "Simultáneo",
                wrong: ["Secuencial", "Consecutivo", "Esporádico"],
            },
            VocabItem {
                prompt: "Descifra las letras: P - E - R - S - E - V - E - R - A - N - C - I - A",
                correct: "Perseverancia",
                wrong: ["Perseveransia", "Preservancia", "Perseverante"],
            },
            VocabItem {
                prompt: "¿Cuál es el antónimo de 'Efímero'?",
                correct: "Perdurable",
                wrong: ["Pasajero", "Fugaz", "Breve"],
            },
            VocabItem {
                prompt: "Identifica la palabra escrita correctamente:",
                correct: "Vicisitud",
                wrong: ["Bicisitud", "Vicisitut", "Bicicitud"],
            },
            VocabItem {
                prompt: "¿Qué palabra significa 'expresado de forma breve y concisa'?",
                correct: "Sucinto",
                wrong: ["Verborreico", "Extenso", "Redundante"],
            },
            VocabItem {
                prompt: "¿Cuál es un sinónimo de 'Ubicuo'?",
                correct: "Omnipresente",
                wrong: ["Escaso", "Raro", "Aislado"],
            },
            VocabItem {
                prompt: "Descifra las letras: B - E - N - E - V - O - L - E - N - C - I - A",
                correct: "Benevolencia",
                wrong: ["Benebolencia", "Benevolensia", "Benevolente"],
            },
        ],
    }
}
