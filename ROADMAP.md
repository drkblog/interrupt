# Interrupt Roadmap

This document outlines the proposed feature enhancements for **Interrupt** to improve user experience, security, and utility.

## Feature Index

| Feature ID | Feature Name | State |
|---|---|---|
| FT-01 | Custom Screensaver Parameter Configuration | Proposed |
| FT-02 | Audio Notifications and Relaxing Chimes | Implemented |
| FT-03 | Local Break Analytics and Compliance Logging | Proposed |
| FT-04 | System Tray Integration and Minimize-to-Tray | Implemented |
| FT-05 | Math Lock Screen | Implemented |
| FT-06 | Vocabulary & Spelling Quiz Lock Screen | Implemented |
| FT-07 | Country Flags & Geography Trivia Lock Screen | Implemented |
| FT-08 | Science & Nature Trivia Lock Screen | Implemented |
| FT-09 | Responsive Centered Info Badges Layout | Implemented |
| FT-10 | English Pronunciation Lock Screen | Implemented |

---

## Proposed Feature Details

### FT-01: Custom Screensaver Parameter Configuration
* **Description**: Enable users to fine-tune visual properties of the screensavers directly from the Settings screen.
* **Benefits**:
  - Customize the breathing rhythm speeds of the Default aurora.
  - Change colors (e.g., green vs. blue Matrix digital rain, custom monochrome clock fonts).
  - Adjust particle counts or character densities for performance tuning.
* **Implementation Plan**: Extend the `ScreensaverComponent` trait to accept dynamic config values, and expose styling widgets under a new "Screensaver Settings" expander in the UI.

### FT-02: Audio Notifications and Relaxing Chimes
* **Description**: Integrate subtle audio feedback to alert users when a state change is imminent or complete.
* **Benefits**:
  - Plays a gentle warn sound when the screen warning banner appears so users aren't startled.
  - Plays a relaxing chime (like a singing bowl or bell) when the break cycle successfully finishes.
* **Implementation Plan**: Integrate a lightweight audio playback crate (such as `rodio` or `kira`) and bundle a few small, high-quality audio files (.wav or .mp3) within the application binary.

### FT-03: Local Break Analytics and Compliance Logging
* **Description**: Save anonymous historical usage patterns to show break compliance trends.
* **Benefits**:
  - Keep track of total screen hours, number of completed breaks, and times the user had to use the override password.
  - Show a small visual chart or progress indicator in the Play UI to gamify break completion.
* **Implementation Plan**: Append records to a simple local file (`%APPDATA%\interrupt\history.json`) and build a tab/view within the settings or main panel to visualize this data using `egui_plot`.

### FT-04: System Tray Integration and Minimize-to-Tray
* **Description**: Allow the application window to be closed or minimized to the Windows System Tray.
* **Benefits**:
  - Keeps the taskbar clean during active working hours.
  - Provides a quick right-click context menu (e.g., "Open Settings", "Lock Now", "Exit").
* **Implementation Plan**: Use a crate like `tray-icon` or `native-dialog` to register a Windows notification tray icon and handle background window minimization.

### FT-05: Math Lock Screen
* **Description**: Introduce a math problem challenge as a requirement/option to unlock or extend the pause screen, encouraging active cognitive breaks or child supervision. Difficulty level, minimum number of questions, and percentage of wait time saved by answering questions are configurable (defaults to 50% wait time saved, with a minimum allowed setting of 30%).
* **Difficulty Levels**:
  - **Low**: Single/double-digit addition and subtraction (e.g., `17 + 8` or `34 - 15`) with totals between 0 and 40.
  - **Medium**: Three digit addition and subtraction, multiplication and division tables or two-step operations (e.g., `12 * 6` or `45 + 5 - 12`).
  - **High**: Multi-step operations with order of operations and division, or simple algebra (e.g., `(15 * 4) / 5 + 18` or solving `4x - 7 = 25`).
* **Implementation Plan**: Define a configuration setting for math lock screen difficulty levels, render a math challenge input block in the pause screen overlay, and only allow unblocking once the correct answer is entered (or the master password is typed).

### FT-06: Vocabulary & Spelling Quiz Lock Screen
* **Description**: Present spelling challenges, letter unscrambles, or vocabulary definition matching (e.g., selecting the correct synonym/antonym or filling in missing letters) tailored for school-aged children.
* **Benefits**:
  - Enhances spelling accuracy, reading comprehension, and language skills.
  - Configurable grade levels (e.g., Early Elementary, Upper Elementary, Middle School).
* **Implementation Plan**: Bundle a curated local dictionary/quiz dataset with categories and grade levels, and render interactive fill-in-the-blank or multiple-choice options on the pause screen.

### FT-07: Country Flags & Geography Trivia Lock Screen
* **Description**: Test geographic knowledge through visual country flag recognition, capital city quizzes, and map location trivia.
* **Benefits**:
  - Promotes global awareness, curiosity about different cultures, and world geography proficiency.
  - Difficulty settings ranging from major world nations/flags to regional geography and capitals.
* **Implementation Plan**: Embed scalable vector flags and country dataset inside binary assets, presenting flag choice cards or capital matching prompts during break sessions.

### FT-08: Science & Nature Trivia Lock Screen
* **Description**: Deliver bite-sized, interactive science quizzes covering topics such as space & astronomy, animal biology, earth science, and physics facts.
* **Benefits**:
  - Encourages scientific curiosity and daily STEM learning.
  - Features explanations after each response to turn break enforcement into a fun learning moment.
* **Implementation Plan**: Maintain a structured JSON file of age-appropriate STEM questions and explanations, allowing random sampling per break with configurable topic filters.

### FT-09: Responsive Centered Info Badges Layout
* **Description**: Dynamically layout main screen bottom information pills into rows that fit the container width, centering each row individually.
* **Benefits**:
  - Prevents horizontal badge overflow on narrow window widths or when running in debug mode or under localized long strings.
  - Maintains clean aesthetic alignment with all rows centered horizontally.
* **Implementation Plan**: Measure text widths via egui painter layout prior to rendering, pack badges into fitted rows, and apply horizontal padding to center each row.

### FT-10: English Pronunciation Lock Screen
* **Description**: Introduce an auditory English pronunciation challenge on the pause screen. The user is presented with three word options while one of the words is pronounced aloud via audio synthesis or pre-recorded clips. The user must listen and select the correct word corresponding to the spoken pronunciation.
* **Difficulty Levels**:
  - **Low (Beginner)**: Simple, phonetically distinct high-frequency words with contrasting vowel and consonant sounds (e.g., `cat`, `dog`, `sun`).
  - **Medium (Intermediate)**: Similar sounding words, minimal pairs, and common homophones (e.g., `ship` / `sheep`, `bare` / `bear`, `desk` / `disk`).
  - **High (Advanced)**: Multi-syllable vocabulary, silent letters, and subtle vowel/consonant distinctions (e.g., `receipt` / `deceit` / `conceit`, `subtle` / `suttle` / `supple`).
  - **Expert**: Complex academic/technical terminology, homographs, fast native speech cadence, and subtle accent/stress variations (e.g., `anemone` / `amnezie` / `aluminum`, `cache` / `cash` / `catch`).
* **Configurable Parameters & Time Savings**:
  - Configurable minimum number of questions required per break session.
  - Configurable percentage of break wait time saved upon successful completion (identical to the Math Lock Screen mechanics).
* **Implementation Plan**: Bundle studio-quality pre-recorded neural audio clips (compressed OGG/MP3 format, ~2-4 MB total asset size) played back via lightweight Rust audio crate (`rodio` / `kira`). Integrate a curated dataset of word option triplets focusing on American English minimal pairs across all four difficulty levels. Render an interactive audio playback/replay button and 3 word choice cards on the pause screen overlay.

