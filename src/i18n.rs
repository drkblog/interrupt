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
            Language::English => "English",
            Language::Spanish => "Spanish",
        }
    }
}

pub fn tr(lang: Language, text: &'static str) -> &'static str {
    if lang == Language::English {
        return text;
    }

    match text {
        // Core App Navigation & Status
        "app_title" => "Interrupt - Control de Pausas de Pantalla",
        "Interrupt Screen Time Manager" => "Interrupt - Control de Pausas de Pantalla",
        "SYSTEM ACTIVE // PLAY MODE" => "SISTEMA ACTIVO // MODO JUEGO",
        "PRE-LOCK WARNING ACTIVE" => "ADVERTENCIA PREVIA AL BLOQUEO ACTIVA",
        "SCREEN BREAK IN PROGRESS" => "DESCANSO DE PANTALLA EN PROGRESO",
        "Active cycle running. Time remaining until next screen lock:" => "Ciclo activo en marcha. Tiempo restante hasta el próximo bloqueo:",
        "Settings open — timer suspended until settings window is closed." => "Configuración abierta — temporizador suspendido hasta cerrar la ventana.",
        "Screen break starting soon!" => "¡El descanso de pantalla comenzará pronto!",
        "Status: Play Mode" => "Estado: Modo Juego",
        "Status: Warning Period" => "Estado: Período de Advertencia",
        "Status: Screen Break" => "Estado: Descanso de Pantalla",
        "Time Remaining:" => "Tiempo Restante:",
        "Play:" => "Juego:",
        "Break:" => "Pausa:",
        "min" => "min",
        "Style:" => "Estilo:",
        "🔒 Lock Now" => "🔒 Bloquear Ahora",
        "🔄 Reset Timer" => "🔄 Reiniciar Temporizador",
        "⚙ Settings" => "⚙ Configuración",
        "🚪 Exit App" => "🚪 Salir de la App",

        // Warning Banner
        "⚠️ SCREEN LOCK WARNING" => "⚠️ ADVERTENCIA DE BLOQUEO DE PANTALLA",
        "Screen will lock in" => "La pantalla se bloqueará en",
        "minutes" => "minutos",

        // Reset Confirmation Dialog
        "Reset Timer Confirmation" => "Confirmación de Reinicio de Temporizador",
        "Enter password to reset the play timer back to zero:" => "Ingrese la contraseña para reiniciar el temporizador de juego:",
        "Confirm" => "Confirmar",
        "Cancel" => "Cancelar",
        "Close" => "Cerrar",
        "Unlock" => "Desbloquear",
        "Incorrect password. Please try again." => "Contraseña incorrecta. Por favor intente de nuevo.",
        "Invalid password." => "Contraseña incorrecta. Por favor intente de nuevo.",

        // Settings Window
        "⚙ Interrupt Settings" => "⚙ Configuración de Interrupt",
        "Authentication Required" => "Autenticación Requerida",
        "Enter current password or master password to access settings:" => "Ingrese la contraseña actual o la contraseña maestra para acceder:",
        "Password..." => "Contraseña...",
        "Unlock Settings" => "Desbloquear Configuración",
        "Invalid password authentication." => "Autenticación de contraseña no válida.",
        "Configure Settings" => "Configurar Opciones",
        "⏸ Timer suspended while settings window is open" => "⏸ Temporizador suspendido mientras la ventana de configuración está abierta",
        "📅 Break Cycles" => "📅 Ciclos de Descanso",
        "🔒 Lock Screen Settings" => "🔒 Opciones de Pantalla de Bloqueo",
        "Interface Language:" => "Idioma de la Interfaz:",
        "Play Time (minutes):" => "Tiempo de Juego (minutos):",
        "Pause Time (minutes):" => "Tiempo de Pausa (minutos):",
        "Warning Time (seconds):" => "Tiempo de Advertencia (segundos):",
        "Warning Sound:" => "Sonido de Advertencia:",
        "▶ Test Sound" => "▶ Probar Sonido",
        "Screensaver Style:" => "Estilo de Salvapantallas:",
        "New Password (optional):" => "Nueva Contraseña (opcional):",
        "Enable Debug Logging:" => "Habilitar Registro de Depuración:",
        "Questions to Solve:" => "Preguntas a Resolver:",
        "Min Break Duration (%):" => "Duración Mínima de Pausa (%):",
        "Difficulty:" => "Dificultad:",
        "This screensaver style has no custom configuration parameters." => "Este estilo de salvapantallas no tiene parámetros personalizados.",
        "💾 Save Settings" => "💾 Guardar Configuración",
        "Settings saved successfully!" => "¡Configuración guardada con éxito!",
        "Failed to save settings" => "Error al guardar la configuración",

        // Screensavers Text & Titles
        "🫁 Inhale deeply..." => "🫁 Inhala profundamente...",
        "⏸️ Hold..." => "⏸️ Mantén el aire...",
        "😮‍💨 Exhale slowly..." => "😮‍💨 Exhala lentamente...",
        "🌿 TIME TO TAKE A BREAK" => "🌿 ES HORA DE TOMAR UN DESCANSO",
        "Step away, stretch, drink water, and rest your eyes." => "Aléjate de la pantalla, estírate, bebe agua y descansa tus ojos.",
        "PAUSE" => "PAUSA",
        "Resting computer screen..." => "Descansando la pantalla...",
        "SYSTEM PAUSED // SCREEN BREAK" => "SISTEMA EN PAUSA // DESCANSO DE PANTALLA",
        "> Stand up and stretch before returning to console." => "> Levántate y estírate antes de volver a la consola.",
        "vocab_title" => "📚 Quiz de Vocabulario y Ortografía",
        "📚 Vocabulary & Spelling Quiz" => "📚 Quiz de Vocabulario y Ortografía",
        "science_title" => "🧪 Trivia de Ciencia y Naturaleza",
        "🧪 Science & Nature Trivia" => "🧪 Trivia de Ciencia y Naturaleza",
        "geography_title" => "🌍 Geografía y Banderas del Mundo",
        "🌍 Geography & Country Flags" => "🌍 Geografía y Banderas del Mundo",
        "math_title" => "🧮 Quiz de Matemáticas",
        "🧮 Math Quiz" => "🧮 Quiz de Matemáticas",

        // Exercise Cards & Quiz Overlay
        "correct_feedback" => "✨ ¡Correcto! Buen trabajo.",
        "incorrect_feedback" => "❌ Incorrecto. ¡Inténtalo de nuevo!",
        "✨ Correct! Great job." => "✨ ¡Correcto! Buen trabajo.",
        "❌ Incorrect. Try again!" => "❌ Incorrecto. ¡Inténtalo de nuevo!",
        "Incorrect choice, try again!" => "Opción incorrecta, ¡inténtalo de nuevo!",
        "Incorrect answer, try again!" => "Respuesta incorrecta, ¡inténtalo de nuevo!",
        "Type answer..." => "Escriba su respuesta...",
        "Submit" => "Enviar",
        "🔑 Use Administrator Password" => "🔑 Usar Contraseña de Administrador",
        "Enter administrator password to unblock immediately:" => "Ingrese la contraseña de administrador para desbloquear inmediatamente:",
        "Unlock Computer" => "Desbloquear Equipo",
        "🎉 All questions solved!" => "🎉 ¡Todas las preguntas resueltas!",
        "🎉 All geography questions solved!" => "🎉 ¡Todas las preguntas de geografía resueltas!",
        "🎉 All STEM questions solved!" => "🎉 ¡Todas las preguntas de ciencia resueltas!",
        "Break must continue to meet the minimum required off-game duration." => "El descanso debe continuar para cumplir la duración mínima requerida sin juego.",
        "Correct! All questions solved! Waiting for break duration..." => "¡Correcto! ¡Todas las preguntas resueltas! Esperando la duración del descanso...",
        "Correct! Excellent job! Next question..." => "¡Correcto! ¡Excelente trabajo! Siguiente pregunta...",

        // Fallback to source English text if not translated
        _ => text,
    }
}

pub struct GeographyItem {
    pub prompt: &'static str,
    pub correct: &'static str,
    pub wrong: [&'static str; 3],
}

pub fn get_geography_question_pool(lang: Language, difficulty: crate::config::GeographyDifficulty) -> &'static [GeographyItem] {
    use crate::config::GeographyDifficulty::*;

    match (lang, difficulty) {
        (Language::Spanish, Low) => &[
            GeographyItem { prompt: "¿Qué país tiene una hoja de arce roja en su bandera?", correct: "Canadá", wrong: ["Estados Unidos", "Australia", "Nueva Zelanda"] },
            GeographyItem { prompt: "¿Qué país tiene un sol rojo sobre fondo blanco en su bandera?", correct: "Japón", wrong: ["China", "Corea del Sur", "Tailandia"] },
            GeographyItem { prompt: "¿Qué país tiene barras y estrellas en su bandera?", correct: "Estados Unidos", wrong: ["Canadá", "Reino Unido", "Australia"] },
            GeographyItem { prompt: "¿Qué país tiene un rombo amarillo sobre fondo verde en su bandera?", correct: "Brasil", wrong: ["Argentina", "Colombia", "Perú"] },
            GeographyItem { prompt: "¿Qué país tiene una bandera tricolor azul, blanca y roja?", correct: "Francia", wrong: ["Italia", "España", "Alemania"] },
            GeographyItem { prompt: "¿Qué país tiene franjas horizontales negra, roja y dorada?", correct: "Alemania", wrong: ["Austria", "Bélgica", "Países Bajos"] },
            GeographyItem { prompt: "¿Qué país tiene la Union Jack y la Cruz del Sur en su bandera?", correct: "Australia", wrong: ["Reino Unido", "Nueva Zelanda", "Fiyi"] },
            GeographyItem { prompt: "¿Qué país tiene un águila devorando una serpiente en su escudo nacional?", correct: "México", wrong: ["España", "Colombia", "Argentina"] },
            GeographyItem { prompt: "¿Cuál es la capital de Francia?", correct: "París", wrong: ["Lyon", "Marsella", "Niza"] },
            GeographyItem { prompt: "¿En qué continente está ubicado Brasil?", correct: "América del Sur", wrong: ["América del Norte", "Europa", "África"] },
            GeographyItem { prompt: "¿Cuál es la capital de Japón?", correct: "Tokio", wrong: ["Kioto", "Osaka", "Yokohama"] },
            GeographyItem { prompt: "¿Cuál es la capital de Estados Unidos?", correct: "Washington D.C.", wrong: ["Nueva York", "Los Ángeles", "Chicago"] },
            GeographyItem { prompt: "¿En qué continente se encuentra Egipto?", correct: "África", wrong: ["Asia", "Europa", "América del Sur"] },
            GeographyItem { prompt: "¿Cuál es la capital de Italia?", correct: "Roma", wrong: ["Milán", "Venecia", "Nápoles"] },
            GeographyItem { prompt: "¿En qué continente está ubicada Australia?", correct: "Oceanía", wrong: ["Europa", "Asia", "África"] },
            GeographyItem { prompt: "¿Cuál es la capital de Alemania?", correct: "Berlín", wrong: ["Múnich", "Fráncfort", "Hamburgo"] },
            GeographyItem { prompt: "¿Cuál es la capital de España?", correct: "Madrid", wrong: ["Barcelona", "Sevilla", "Valencia"] },
            GeographyItem { prompt: "¿Cuál es la capital del Reino Unido?", correct: "Londres", wrong: ["Edimburgo", "Dublín", "Mánchester"] },
        ],
        (Language::Spanish, Medium) => &[
            GeographyItem { prompt: "¿Qué país tiene un taegeuk (círculo rojo y azul) en su bandera?", correct: "Corea del Sur", wrong: ["Japón", "China", "Vietnam"] },
            GeographyItem { prompt: "¿Qué país tiene un Sol de Mayo sobre franjas celestes y blancas?", correct: "Argentina", wrong: ["Uruguay", "Chile", "Brasil"] },
            GeographyItem { prompt: "¿Qué país tiene la rueda Ashoka Chakra en su bandera tricolor?", correct: "India", wrong: ["Pakistán", "Bangladés", "Sri Lanka"] },
            GeographyItem { prompt: "¿Qué país tiene una cruz azul sobre fondo amarillo en su bandera?", correct: "Suecia", wrong: ["Noruega", "Finlandia", "Dinamarca"] },
            GeographyItem { prompt: "¿Qué país tiene franjas azules y blancas con una cruz en la esquina?", correct: "Grecia", wrong: ["Italia", "Turquía", "Chipre"] },
            GeographyItem { prompt: "¿Cuál es la capital de Argentina?", correct: "Buenos Aires", wrong: ["Córdoba", "Rosario", "Mendoza"] },
            GeographyItem { prompt: "¿Cuál es la capital de Canadá?", correct: "Ottawa", wrong: ["Toronto", "Montreal", "Vancouver"] },
            GeographyItem { prompt: "¿En qué continente se encuentra la India?", correct: "Asia", wrong: ["Europa", "África", "Oceanía"] },
            GeographyItem { prompt: "¿Cuál es la capital de Corea del Sur?", correct: "Seúl", wrong: ["Busan", "Incheon", "Daegu"] },
            GeographyItem { prompt: "¿Cuál es la capital de México?", correct: "Ciudad de México", wrong: ["Guadalajara", "Monterrey", "Cancún"] },
            GeographyItem { prompt: "¿Cuál es la capital de Grecia?", correct: "Atenas", wrong: ["Tesalónica", "Heraclión", "Patras"] },
            GeographyItem { prompt: "¿Cuál es la capital de Suecia?", correct: "Estocolmo", wrong: ["Gotemburgo", "Malmö", "Uppsala"] },
            GeographyItem { prompt: "¿Cuál es la capital de Tailandia?", correct: "Bangkok", wrong: ["Chiang Mai", "Phuket", "Pattaya"] },
            GeographyItem { prompt: "¿Cuál es la capital de Egipto?", correct: "El Cairo", wrong: ["Alejandría", "Giza", "Lúxor"] },
            GeographyItem { prompt: "¿Cuál es la capital de Noruega?", correct: "Oslo", wrong: ["Bergen", "Trondheim", "Stavanger"] },
        ],
        (Language::Spanish, High) => &[
            GeographyItem { prompt: "¿Qué país tiene la única bandera nacional no rectangular?", correct: "Nepal", wrong: ["Bután", "India", "Myanmar"] },
            GeographyItem { prompt: "¿Qué país tiene una cruz blanca sobre fondo cuadrado rojo?", correct: "Suiza", wrong: ["Austria", "Dinamarca", "Suecia"] },
            GeographyItem { prompt: "¿Qué país tiene una luna creciente y estrella blancas sobre fondo rojo?", correct: "Turquía", wrong: ["Grecia", "Egipto", "Túnez"] },
            GeographyItem { prompt: "¿Qué país tiene una franja en forma de Y y seis colores en su bandera?", correct: "Sudáfrica", wrong: ["Kenia", "Nigeria", "Zimbabue"] },
            GeographyItem { prompt: "¿Cuál es la capital de Australia?", correct: "Canberra", wrong: ["Sídney", "Melbourne", "Brisbane"] },
            GeographyItem { prompt: "¿Cuál es la capital de Brasil?", correct: "Brasilia", wrong: ["Río de Janeiro", "San Pablo", "Salvador"] },
            GeographyItem { prompt: "¿Cuál es la capital de Kazajistán?", correct: "Astana", wrong: ["Almaty", "Shymkent", "Karaganda"] },
            GeographyItem { prompt: "¿Cuál es la capital de Kenia?", correct: "Nairobi", wrong: ["Mombasa", "Kisumu", "Nakuru"] },
            GeographyItem { prompt: "¿Cuál es la capital de Uruguay?", correct: "Montevideo", wrong: ["Salto", "Ciudad de la Costa", "Paysandú"] },
            GeographyItem { prompt: "¿Cuál es la capital de Madagascar?", correct: "Antananarivo", wrong: ["Toamasina", "Antsirabe", "Mahajanga"] },
            GeographyItem { prompt: "¿Cuál es la capital de Nepal?", correct: "Katmandú", wrong: ["Pokhara", "Lalitpur", "Bharatpur"] },
            GeographyItem { prompt: "¿Cuál es la capital de Estonia?", correct: "Tallin", wrong: ["Tartu", "Narva", "Pärnu"] },
            GeographyItem { prompt: "¿Cuál es la capital de Marruecos?", correct: "Rabat", wrong: ["Casablanca", "Marrakech", "Fez"] },
            GeographyItem { prompt: "¿Cuál es la capital de Suiza?", correct: "Berna", wrong: ["Zúrich", "Ginebra", "Basilea"] },
            GeographyItem { prompt: "¿Cuál es la capital de Turquía?", correct: "Ankara", wrong: ["Estambul", "Esmirna", "Bursa"] },
            GeographyItem { prompt: "¿Cuál es la capital de Nueva Zelanda?", correct: "Wellington", wrong: ["Auckland", "Christchurch", "Hamilton"] },
        ],
        (_, Low) => &[
            GeographyItem { prompt: "Which country's flag features a red maple leaf?", correct: "Canada", wrong: ["United States", "Australia", "New Zealand"] },
            GeographyItem { prompt: "Which country's flag features a red sun on a white field?", correct: "Japan", wrong: ["China", "South Korea", "Thailand"] },
            GeographyItem { prompt: "Which country's flag features stars and stripes?", correct: "United States", wrong: ["Canada", "United Kingdom", "Australia"] },
            GeographyItem { prompt: "Which country's flag features a green field with a yellow diamond?", correct: "Brazil", wrong: ["Argentina", "Colombia", "Peru"] },
            GeographyItem { prompt: "Which country's flag features a blue, white, and red vertical tricolor?", correct: "France", wrong: ["Italy", "Spain", "Germany"] },
            GeographyItem { prompt: "Which country's flag features horizontal stripes of black, red, and gold?", correct: "Germany", wrong: ["Austria", "Belgium", "Netherlands"] },
            GeographyItem { prompt: "Which country's flag features a Union Jack and Southern Cross stars?", correct: "Australia", wrong: ["United Kingdom", "New Zealand", "Fiji"] },
            GeographyItem { prompt: "Which country's flag features an eagle perched on a cactus eating a snake?", correct: "Mexico", wrong: ["Spain", "Colombia", "Argentina"] },
            GeographyItem { prompt: "What is the capital of France?", correct: "Paris", wrong: ["Lyon", "Marseille", "Nice"] },
            GeographyItem { prompt: "Which continent is Brazil located in?", correct: "South America", wrong: ["North America", "Europe", "Africa"] },
            GeographyItem { prompt: "What is the capital of Japan?", correct: "Tokyo", wrong: ["Kyoto", "Osaka", "Yokohama"] },
            GeographyItem { prompt: "What is the capital of the United States?", correct: "Washington, D.C.", wrong: ["New York", "Los Angeles", "Chicago"] },
            GeographyItem { prompt: "Which continent is Egypt located in?", correct: "Africa", wrong: ["Asia", "Europe", "South America"] },
            GeographyItem { prompt: "What is the capital of Italy?", correct: "Rome", wrong: ["Milan", "Venice", "Naples"] },
            GeographyItem { prompt: "Which continent is Australia located in?", correct: "Oceania", wrong: ["Europe", "Asia", "Africa"] },
            GeographyItem { prompt: "What is the capital of Germany?", correct: "Berlin", wrong: ["Munich", "Frankfurt", "Hamburg"] },
            GeographyItem { prompt: "What is the capital of Spain?", correct: "Madrid", wrong: ["Barcelona", "Seville", "Valencia"] },
            GeographyItem { prompt: "What is the capital of the United Kingdom?", correct: "London", wrong: ["Edinburgh", "Dublin", "Manchester"] },
        ],
        (_, Medium) => &[
            GeographyItem { prompt: "Which country's flag features a taegeuk (red and blue circle) and trigrams?", correct: "South Korea", wrong: ["Japan", "China", "Vietnam"] },
            GeographyItem { prompt: "Which country's flag features a Sun of May on light blue and white stripes?", correct: "Argentina", wrong: ["Uruguay", "Chile", "Brazil"] },
            GeographyItem { prompt: "Which country's flag features the Ashoka Chakra wheel on a tricolor?", correct: "India", wrong: ["Pakistan", "Bangladesh", "Sri Lanka"] },
            GeographyItem { prompt: "Which country's flag features a yellow cross on a blue field?", correct: "Sweden", wrong: ["Norway", "Finland", "Denmark"] },
            GeographyItem { prompt: "Which country's flag features blue and white stripes with a cross in the corner?", correct: "Greece", wrong: ["Italy", "Turkey", "Cyprus"] },
            GeographyItem { prompt: "What is the capital of Argentina?", correct: "Buenos Aires", wrong: ["Cordoba", "Rosario", "Mendoza"] },
            GeographyItem { prompt: "What is the capital of Canada?", correct: "Ottawa", wrong: ["Toronto", "Montreal", "Vancouver"] },
            GeographyItem { prompt: "Which continent is India located in?", correct: "Asia", wrong: ["Europe", "Africa", "Oceania"] },
            GeographyItem { prompt: "What is the capital of South Korea?", correct: "Seoul", wrong: ["Busan", "Incheon", "Daegu"] },
            GeographyItem { prompt: "What is the capital of Mexico?", correct: "Mexico City", wrong: ["Guadalajara", "Monterrey", "Cancun"] },
            GeographyItem { prompt: "What is the capital of Greece?", correct: "Athens", wrong: ["Thessaloniki", "Heraklion", "Patras"] },
            GeographyItem { prompt: "What is the capital of Sweden?", correct: "Stockholm", wrong: ["Gothenburg", "Malmo", "Uppsala"] },
            GeographyItem { prompt: "What is the capital of Thailand?", correct: "Bangkok", wrong: ["Chiang Mai", "Phuket", "Pattaya"] },
            GeographyItem { prompt: "What is the capital of Egypt?", correct: "Cairo", wrong: ["Alexandria", "Giza", "Luxor"] },
            GeographyItem { prompt: "What is the capital of Norway?", correct: "Oslo", wrong: ["Bergen", "Trondheim", "Stavanger"] },
        ],
        (_, High) => &[
            GeographyItem { prompt: "Which country has the world's only non-rectangular national flag?", correct: "Nepal", wrong: ["Bhutan", "India", "Myanmar"] },
            GeographyItem { prompt: "Which country's flag features a white cross on a square red field?", correct: "Switzerland", wrong: ["Austria", "Denmark", "Sweden"] },
            GeographyItem { prompt: "Which country's flag features a white crescent moon and star on a red field?", correct: "Turkey", wrong: ["Greece", "Egypt", "Tunisia"] },
            GeographyItem { prompt: "Which country's flag features a Y-shaped green band and six colors?", correct: "South Africa", wrong: ["Kenya", "Nigeria", "Zimbabwe"] },
            GeographyItem { prompt: "What is the capital of Australia?", correct: "Canberra", wrong: ["Sydney", "Melbourne", "Brisbane"] },
            GeographyItem { prompt: "What is the capital of Brazil?", correct: "Brasilia", wrong: ["Rio de Janeiro", "Sao Paulo", "Salvador"] },
            GeographyItem { prompt: "What is the capital of Kazakhstan?", correct: "Astana", wrong: ["Almaty", "Shymkent", "Karaganda"] },
            GeographyItem { prompt: "What is the capital of Kenya?", correct: "Nairobi", wrong: ["Mombasa", "Kisumu", "Nakuru"] },
            GeographyItem { prompt: "What is the capital of Uruguay?", correct: "Montevideo", wrong: ["Salto", "Ciudad de la Costa", "Paysandu"] },
            GeographyItem { prompt: "What is the capital of Madagascar?", correct: "Antananarivo", wrong: ["Toamasina", "Antsirabe", "Mahajanga"] },
            GeographyItem { prompt: "What is the capital of Nepal?", correct: "Kathmandu", wrong: ["Pokhara", "Lalitpur", "Bharatpur"] },
            GeographyItem { prompt: "What is the capital of Estonia?", correct: "Tallinn", wrong: ["Tartu", "Narva", "Parnu"] },
            GeographyItem { prompt: "What is the capital of Morocco?", correct: "Rabat", wrong: ["Casablanca", "Marrakesh", "Fes"] },
            GeographyItem { prompt: "What is the capital of Switzerland?", correct: "Bern", wrong: ["Zurich", "Geneva", "Basel"] },
            GeographyItem { prompt: "What is the capital of Turkey?", correct: "Ankara", wrong: ["Istanbul", "Izmir", "Bursa"] },
            GeographyItem { prompt: "What is the capital of New Zealand?", correct: "Wellington", wrong: ["Auckland", "Christchurch", "Hamilton"] },
        ],
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

pub struct ScienceItem {
    pub prompt: &'static str,
    pub correct: &'static str,
    pub wrong: [&'static str; 3],
    pub explanation: &'static str,
}

pub fn get_science_question_pool(lang: Language, difficulty: crate::config::ScienceDifficulty) -> &'static [ScienceItem] {
    use crate::config::ScienceDifficulty::*;

    match (lang, difficulty) {
        (Language::English, Low) => &[
            ScienceItem {
                prompt: "Which planet is known as the Red Planet?",
                correct: "Mars 🔴",
                wrong: ["Venus ♀️", "Jupiter ♃", "Saturn ♄"],
                explanation: "💡 Did you know? Mars appears red due to iron oxide (rust) on its surface.",
            },
            ScienceItem {
                prompt: "What process do plants use to convert sunlight into energy?",
                correct: "Photosynthesis 🍃",
                wrong: ["Respiration", "Digestion", "Fermentation"],
                explanation: "💡 Did you know? Photosynthesis produces oxygen, which animals and humans breathe.",
            },
            ScienceItem {
                prompt: "What is the hardest natural substance on Earth?",
                correct: "Diamond 💎",
                wrong: ["Gold", "Granite", "Quartz"],
                explanation: "💡 Did you know? Diamonds are formed deep within Earth under extreme pressure and heat.",
            },
            ScienceItem {
                prompt: "Which force pulls objects toward the center of Earth?",
                correct: "Gravity 🍏",
                wrong: ["Magnetism", "Friction", "Tension"],
                explanation: "💡 Did you know? Sir Isaac Newton formulated the law of universal gravitation in 1687.",
            },
            ScienceItem {
                prompt: "Which mammal is capable of true sustained flight?",
                correct: "Bat 🦇",
                wrong: ["Flying Squirrel", "Sugar Glider", "Penguin"],
                explanation: "💡 Did you know? Bats are the only mammals naturally capable of powered flight.",
            },
            ScienceItem {
                prompt: "What gas do humans need to breathe to survive?",
                correct: "Oxygen 💨",
                wrong: ["Carbon Dioxide", "Nitrogen", "Helium"],
                explanation: "💡 Did you know? Oxygen makes up about 21% of Earth's atmosphere.",
            },
            ScienceItem {
                prompt: "What is the center of an atom called?",
                correct: "Nucleus ⚛️",
                wrong: ["Electron", "Proton", "Orbit"],
                explanation: "💡 Did you know? The nucleus contains protons and neutrons, holding almost all of the atom's mass.",
            },
            ScienceItem {
                prompt: "Which giant star is at the center of our solar system?",
                correct: "The Sun ☀️",
                wrong: ["Sirius", "Proxima Centauri", "Betelgeuse"],
                explanation: "💡 Did you know? The Sun contains 99.8% of the total mass of the solar system.",
            },
        ],
        (Language::English, Medium) => &[
            ScienceItem {
                prompt: "What is the largest planet in our solar system?",
                correct: "Jupiter 🪐",
                wrong: ["Saturn", "Neptune", "Uranus"],
                explanation: "💡 Did you know? Jupiter is so large that over 1,300 Earths could fit inside it.",
            },
            ScienceItem {
                prompt: "What is the chemical formula for water?",
                correct: "H₂O 💧",
                wrong: ["CO₂", "NaCl", "O₂"],
                explanation: "💡 Did you know? Water molecules consist of two hydrogen atoms bonded to one oxygen atom.",
            },
            ScienceItem {
                prompt: "Which layer of the atmosphere contains most of Earth's weather?",
                correct: "Troposphere 🌤️",
                wrong: ["Stratosphere", "Mesosphere", "Thermosphere"],
                explanation: "💡 Did you know? The troposphere extends up to about 8 to 15 kilometers above sea level.",
            },
            ScienceItem {
                prompt: "Which organ in the human body pumps blood through the circulatory system?",
                correct: "Heart 🫀",
                wrong: ["Lungs", "Liver", "Brain"],
                explanation: "💡 Did you know? The human heart beats about 100,000 times per day.",
            },
            ScienceItem {
                prompt: "What unit is used to measure electrical current?",
                correct: "Ampere (Amp) ⚡",
                wrong: ["Volt", "Watt", "Ohm"],
                explanation: "💡 Did you know? Named after André-Marie Ampère, pioneer in electromagnetism.",
            },
            ScienceItem {
                prompt: "What is the process by which liquid water turns into water vapor?",
                correct: "Evaporation ☁️",
                wrong: ["Condensation", "Precipitation", "Sublimation"],
                explanation: "💡 Did you know? Evaporation is a key phase in Earth's water cycle driven by solar heat.",
            },
            ScienceItem {
                prompt: "Which planet is famous for its prominent ring system?",
                correct: "Saturn 🪐",
                wrong: ["Mars", "Mercury", "Venus"],
                explanation: "💡 Did you know? Saturn's rings are made mostly of ice particles and space dust.",
            },
        ],
        (Language::English, High) => &[
            ScienceItem {
                prompt: "What is the speed of light in a vacuum approximately?",
                correct: "300,000 km/s ⚡",
                wrong: ["150,000 km/s", "1,000,000 km/s", "30,000 km/s"],
                explanation: "💡 Did you know? Light takes about 8 minutes and 20 seconds to travel from the Sun to Earth.",
            },
            ScienceItem {
                prompt: "Which cell organelle is known as the powerhouse of the cell?",
                correct: "Mitochondria 🔬",
                wrong: ["Nucleus", "Ribosome", "Golgi Apparatus"],
                explanation: "💡 Did you know? Mitochondria generate most of the ATP energy powering biochemical reactions.",
            },
            ScienceItem {
                prompt: "What type of celestial object is a pulsar?",
                correct: "Neutron Star 💫",
                wrong: ["Black Hole", "White Dwarf", "Red Giant"],
                explanation: "💡 Did you know? Pulsars are rapidly rotating neutron stars emitting beams of radiation.",
            },
            ScienceItem {
                prompt: "What is the boundary between Earth's crust and mantle called?",
                correct: "Moho Discontinuity 🌋",
                wrong: ["Gutenberg Discontinuity", "Conrad Discontinuity", "Lehmann Discontinuity"],
                explanation: "💡 Did you know? Discovered by seismologist Andrija Mohorovičić in 1909.",
            },
            ScienceItem {
                prompt: "Which chemical element has the atomic number 1?",
                correct: "Hydrogen ⚛️",
                wrong: ["Helium", "Lithium", "Carbon"],
                explanation: "💡 Did you know? Hydrogen makes up roughly 75% of all baryonic mass in the universe.",
            },
            ScienceItem {
                prompt: "What law of thermodynamics states that energy cannot be created or destroyed?",
                correct: "First Law (Conservation) ⚖️",
                wrong: ["Second Law (Entropy)", "Third Law (Absolute Zero)", "Zeroth Law"],
                explanation: "💡 Did you know? Energy can only change form, such as chemical energy turning into heat.",
            },
        ],
        (Language::Spanish, Low) => &[
            ScienceItem {
                prompt: "¿Qué planeta es conocido como el Planeta Rojo?",
                correct: "Marte 🔴",
                wrong: ["Venus ♀️", "Júpiter ♃", "Saturno ♄"],
                explanation: "💡 ¿Sabías que? Marte se ve rojo debido al óxido de hierro (herrumbre) en su superficie.",
            },
            ScienceItem {
                prompt: "¿Qué proceso usan las plantas para convertir la luz solar en energía?",
                correct: "Fotosíntesis 🍃",
                wrong: ["Respiración", "Digestión", "Fermentación"],
                explanation: "💡 ¿Sabías que? La fotosíntesis produce oxígeno, el cual respiran los animales y humanos.",
            },
            ScienceItem {
                prompt: "¿Cuál es la sustancia natural más dura de la Tierra?",
                correct: "Diamante 💎",
                wrong: ["Oro", "Granito", "Cuarzo"],
                explanation: "💡 ¿Sabías que? Los diamantes se forman en las profundidades de la Tierra bajo extrema presión.",
            },
            ScienceItem {
                prompt: "¿Qué fuerza atrae los objetos hacia el centro de la Tierra?",
                correct: "Gravedad 🍏",
                wrong: ["Magnetismo", "Fricción", "Tensión"],
                explanation: "💡 ¿Sabías que? Sir Isaac Newton formuló la ley de la gravitación universal en 1687.",
            },
            ScienceItem {
                prompt: "¿Qué mamífero es capaz de volar de forma continua y sostenida?",
                correct: "Murciélago 🦇",
                wrong: ["Ardilla Voladora", "Petauro del Azúcar", "Pingüino"],
                explanation: "💡 ¿Sabías que? Los murciélagos son los únicos mamíferos capaces de vuelo activo.",
            },
            ScienceItem {
                prompt: "¿Qué gas necesitamos respirar los humanos para vivir?",
                correct: "Oxígeno 💨",
                wrong: ["Dióxido de Carbono", "Nitrógeno", "Helio"],
                explanation: "💡 ¿Sabías que? El oxígeno compone aproximadamente el 21% de la atmósfera terrestre.",
            },
            ScienceItem {
                prompt: "¿Cómo se llama el centro de un átomo?",
                correct: "Núcleo ⚛️",
                wrong: ["Electrón", "Protón", "Órbita"],
                explanation: "💡 ¿Sabías que? El núcleo contiene protones y neutrones y alberga casi toda la masa del átomo.",
            },
            ScienceItem {
                prompt: "¿Qué estrella gigante se encuentra en el centro de nuestro sistema solar?",
                correct: "El Sol ☀️",
                wrong: ["Sirio", "Próxima Centauri", "Betelgeuse"],
                explanation: "💡 ¿Sabías que? El Sol contiene el 99.8% de toda la masa del sistema solar.",
            },
        ],
        (Language::Spanish, Medium) => &[
            ScienceItem {
                prompt: "¿Cuál es el planeta más grande de nuestro sistema solar?",
                correct: "Júpiter 🪐",
                wrong: ["Saturno", "Neptuno", "Urano"],
                explanation: "💡 ¿Sabías que? Júpiter es tan grande que dentro de él cabrían más de 1,300 Tierras.",
            },
            ScienceItem {
                prompt: "¿Cuál es la fórmula química del agua?",
                correct: "H₂O 💧",
                wrong: ["CO₂", "NaCl", "O₂"],
                explanation: "💡 ¿Sabías que? Cada molécula de agua tiene dos átomos de hidrógeno y uno de oxígeno.",
            },
            ScienceItem {
                prompt: "¿En qué capa de la atmósfera ocurren la mayoría de los fenómenos meteorológicos?",
                correct: "Troposfera 🌤️",
                wrong: ["Estratosfera", "Mesosfera", "Termosfera"],
                explanation: "💡 ¿Sabías que? La troposfera se extiende desde la superficie hasta los 8-15 km de altura.",
            },
            ScienceItem {
                prompt: "¿Qué órgano del cuerpo humano bombea sangre a todo el sistema circulatorio?",
                correct: "Corazón 🫀",
                wrong: ["Pulmones", "Hígado", "Cerebro"],
                explanation: "💡 ¿Sabías que? El corazón humano late unas 100,000 veces al día.",
            },
            ScienceItem {
                prompt: "¿Qué unidad se utiliza para medir la corriente eléctrica?",
                correct: "Amperio (Amp) ⚡",
                wrong: ["Voltio", "Vatio", "Ohmio"],
                explanation: "💡 ¿Sabías que? Lleva el nombre de André-Marie Ampère, pionero del electromagnetismo.",
            },
            ScienceItem {
                prompt: "¿Cómo se llama el proceso por el cual el agua líquida se convierte en vapor?",
                correct: "Evaporación ☁️",
                wrong: ["Condensación", "Precipitación", "Sublimación"],
                explanation: "💡 ¿Sabías que? La evaporación es impulsada por el calor del Sol en el ciclo del agua.",
            },
            ScienceItem {
                prompt: "¿Qué planeta es famoso por sus deslumbrantes anillos?",
                correct: "Saturno 🪐",
                wrong: ["Marte", "Mercurio", "Venus"],
                explanation: "💡 ¿Sabías que? Los anillos de Saturno están formados principalmente por partículas de hielo y polvo.",
            },
        ],
        (Language::Spanish, High) => &[
            ScienceItem {
                prompt: "¿Cuál es la velocidad aproximada de la luz en el vacío?",
                correct: "300,000 km/s ⚡",
                wrong: ["150,000 km/s", "1,000,000 km/s", "30,000 km/s"],
                explanation: "💡 ¿Sabías que? La luz del Sol tarda unos 8 minutos y 20 segundos en llegar a la Tierra.",
            },
            ScienceItem {
                prompt: "¿Qué orgánulo celular es conocido como la central de energía de la célula?",
                correct: "Mitocondria 🔬",
                wrong: ["Núcleo", "Ribosoma", "Aparato de Golgi"],
                explanation: "💡 ¿Sabías que? Las mitocondrias generan la mayor parte del ATP usado en reacciones celulares.",
            },
            ScienceItem {
                prompt: "¿Qué tipo de objeto celeste es un púlsar?",
                correct: "Estrella de Neutrones 💫",
                wrong: ["Agujero Negro", "Enana Blanca", "Gigante Roja"],
                explanation: "💡 ¿Sabías que? Los púlsares son estrellas de neutrones en rápida rotación que emiten radiación.",
            },
            ScienceItem {
                prompt: "¿Cómo se llama la frontera entre la corteza terrestre y el manto?",
                correct: "Discontinuidad de Mohorovičić 🌋",
                wrong: ["Discontinuidad de Gutenberg", "Discontinuidad de Conrad", "Discontinuidad de Lehmann"],
                explanation: "💡 ¿Sabías que? Fue descubierta por el sismólogo croata Andrija Mohorovičić en 1909.",
            },
            ScienceItem {
                prompt: "¿Qué elemento químico tiene el número atómico 1?",
                correct: "Hidrógeno ⚛️",
                wrong: ["Helio", "Litio", "Carbono"],
                explanation: "💡 ¿Sabías que? El hidrógeno representa aproximadamente el 75% de la masa bariónica del universo.",
            },
            ScienceItem {
                prompt: "¿Qué ley de la termodinámica establece que la energía no se crea ni se destruye?",
                correct: "Primera Ley (Conservación) ⚖️",
                wrong: ["Segunda Ley (Entropía)", "Tercera Ley (Cero Absoluto)", "Ley Cero"],
                explanation: "💡 ¿Sabías que? La energía solo se transforma, por ejemplo de energía química a calor.",
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_names() {
        assert_eq!(Language::English.name(), "English");
        assert_eq!(Language::Spanish.name(), "Spanish");
    }

    #[test]
    fn test_tr_english_passthrough() {
        assert_eq!(
            tr(Language::English, "Interrupt Screen Time Manager"),
            "Interrupt Screen Time Manager"
        );
        assert_eq!(tr(Language::English, "🔒 Lock Now"), "🔒 Lock Now");
        assert_eq!(tr(Language::English, "Unmapped Test Key"), "Unmapped Test Key");
    }

    #[test]
    fn test_tr_spanish_translation() {
        assert_eq!(
            tr(Language::Spanish, "Interrupt Screen Time Manager"),
            "Interrupt - Control de Pausas de Pantalla"
        );
        assert_eq!(tr(Language::Spanish, "🔒 Lock Now"), "🔒 Bloquear Ahora");
        assert_eq!(tr(Language::Spanish, "⚙ Settings"), "⚙ Configuración");
        assert_eq!(
            tr(Language::Spanish, "Reset Timer Confirmation"),
            "Confirmación de Reinicio de Temporizador"
        );
    }

    #[test]
    fn test_tr_spanish_unmapped_fallback() {
        assert_eq!(
            tr(Language::Spanish, "Unmapped Custom Text"),
            "Unmapped Custom Text"
        );
    }
}
