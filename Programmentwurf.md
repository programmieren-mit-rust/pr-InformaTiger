# Bildersuchmachine für PNGs

Als Programmentwurf ist eine simple Bildersuchmaschine zu erstellen. Als Suchbegriff nimmt die Suchmaschine ist ebenfalls ein Bild entgegen. 
Die Suchmaschine wird die ähnlichsten Bilder aus einem Pool vorher indizierter Bilder ausgeben. Das Ähnlichkeitskriterium ist flexibel definierbar.

Die Funktionalität wird als Bibliothek bereit gestellt. Die Bibliothek beinhaltet zentrale Typen die

- Bilder,
- den Suchindex und
- Features inklusive zugehöriger Ähnlichkeitsmaße

repräsentieren. Benutzer der Bibliothek können aus der Bibliothek bereitgestellte Features mit dem Suchindex verwenden oder eigene implementieren und mit
dem Suchindex verwenden.

## Begrifferklärungen

### Bilder

Bilder bestehen aus Pixeln und können als Matrix repräsentiert werden. Jeder Pixel hat eine Zeilenkoordinate, eine Spaltenkoordinate und einen Pixelwert.
Im Falle eines Grauwertbildes sind die Pixelwerte die Intensitäten im Bild. Oft haben Pixelwerte den Datentyp `u8` und befinden sich zwischen `0` 
und `255`. Zum Berechnen von Bildeigenschaften, auch Features genannt, eignen sich oft `f32`-Pixelwerte in `[0, 1]` besser. 
[PNG](https://de.wikipedia.org/wiki/Portable_Network_Graphics) Bilder haben üblicherweise
3 oder 4 Kanäle. Die entsprechenden Farbformate heißen RGB bzw. RGBA. Jeder Kanal entspricht einem Grauwertbild. Kanal 0 entspricht der Farbe Rot, 
Kanal 1 der Farbe Grün und Kanal 2 der Farbe Blau. Wenn ein Grauwertbild mehrere 3 Kanäle hat, sind die Intensitäten aller Farbkanäle identisch.
Ein komplett rotes Bild hat im 255 überall im roten Kanal und 0 im blauen und roten Kanal.
Falls es einen 4ten Kanal gibt, ist das der Alpha-Kanal, der die Transparenz des jeweiligen Pixels bestimmt.

### Ähnlichkeiten

Ähnlichkeit zwischen Bildern lässt sich auf verschiedene Arten quantifizeren. Beispielsweise könnte man sagen, 2 Bilder sind ähnlich, 
wenn sie ähnlich hell sind. Man könnte auch sagen, 2 Bilder sind ähnlich, wenn sie ähnliche Verteilungen von Farbwerten haben. Eine
Alternative wäre, dass Bilder ähnlich sind, wenn ähnliche Objekte enthalten sind. Durchschnittliche Helligkeit, Verteilung von Farbwerten 
oder enthaltene Objekte sind Eigenschaften, die wir Bildern zuweisen können. Wir nennen diese Eigenschaften auch Features. Da die Features unterschiedlich in ihrer Struktur und Bedeutung sein können, geben wir zu jedem Feature auch ein passende Ähnlichkeitsfunktion an.

### Suchindex

Ein zentraler Bestandteil der Suchmaschine ist der Suchindex. Wenn wir ein Bild aus unserem Pool suchen, das unserer Suchanfrage ähnlich ist,
wollen wir vermeiden für alle Bilder im Pool die Features neu zu berechnen. Anstatt dessen werden alle werden alle Features für die Bilder im 
Pool vorberechnet und im Suchindex gespeichert. Diese Schritt der Vorberechnung nennt sich Indexierung.

## Aufgabe

### 1 Bildtypen

Erstellt einen oder mehrere Typen, die Bilder repräsentieren. Die Bildtypen sollen sowohl die Meta-Informationen Anzahl der Zeilen, 
Anzahl der Spalten und Anzahl der Kanäle beinhalten, als auch die eigentlichen Bilddaten. Es muss möglich sein Bilder mit Datentypen
`u8` und `f32` zu repräsentieren. Verwende den Crate [PNG](https://crates.io/crates/png) zum Laden von Bildern im PNG-Format.

### 2 Features

Stellt folgende Features bereit.

#### 2a Mittlere Helligkeit

Falls das Bild ein `u8`-Farbbild ist, wandeln wir es erst in ein `f32`-Grauwertbild um.
Die Intensität $I_{i,j}$ im Grauwertbild für den Pixel in Zeile $i$ und Spalte $j$ berechnet sich durch

$$I_{i,j}=\frac{0.3R_{i,j}+0.59G_{i,j}+0.11B_{i,j}}{255}$$

wobei $R_{i,j}$ der Wert des Rotkanals, $G_{i,j}$ der Wert des Grünkanals und $B_{i,j}$ der Wert des
Blaukanals für den Pixel $(i,j)$ im Ausgangsbild ist.

Ausgehend vom Bild der Grauwertintensitäten $I$ mit $m$ Zeilen und $n$ Spalten bestimmen wir die mittlere Helligkeit durch

$$I_{\text{mean}} = \sum_{i=1}^n\sum_{j=1}^m I_{i,j}/(nm).$$

Als Ähnlichkeitsmaß bietet sich $1-I_{\text{mean}}$ an, wenn $1$ der maximale Wert des Wertebereichs der Bildwerte ist.

#### 2b Farbhistogramme

Das Histogramm eines Farbkanals quantifiziert die Verteilung seiner Intensitäten. Um ein Histogramm zu berechnen, unterteilt man
den Wertebereich $[v_{\text{min}}, v_{\text{max}}]\subset\mathbb R$ in $n$ gleich große Unterbereiche 
der Länge $\ell=(v_{\text{max}}-v_{\text{min}})/n$. Das Histogramm ist ein Vektor der Länge $n$ wobei jede Komponente die Anzahl der
Pixel beinhaltet, deren Wert im entsprechenden Unterbereich liegt.

Implementiere das Feature mit $n=5$ für die Farbkanäle R, G und B und hänge die Histogramme in einen 15-elementigen Feature-Vektor aneinander. Als 
Ähnlichkeitsmaß bietet sich die [Kosinusähnlichkeit](https://de.wikipedia.org/wiki/Kosinus-Ähnlichkeit) an.

### 3 Suchindex

Erstelle für den Suchindex einen Typ, der für einen gegebenen Pool an Bildern auf der lokalen Festplatte die Features vorberechnet und serializiert.
Verwende zum Serialisieren des Indexes als `*.json` die Crates [`serde`](https://crates.io/crates/serde) und 
[`serde_json`](https://crates.io/crates/serde_json). Dazu reicht es, für den entsprechenden Typ
`#[derive(Serialize, Deserialize)]` zu verwenden. Speichern kannst du die Variable, nennen wir sie `search_index`, des entsprechenden Typen z.B. mit
```rust
let data_str = serde_json::to_string(&search_index)?;
fs::write(filename, data_str)?;
```
und laden kannst du z.B. mit
```rust
let data_str = fs::read_to_string(path)?;
serde_json::from_str(&data_str)?;
```
Der Benutzer des Suchindex soll entscheiden können, welche Features zur Indizierung verwendet werden. Das können die von dir bereitgestellten sein oder 
selbst implementierte.

### 4 Parallelisierung

Parallelisiere die Erstellung die Indexes durch die Verwendung von Threads und Message Passing. Bonuspunkte gibt es für
die Implementierung eines Threadpools, bei dem man die maximale Anzahl verwendeter Threads beschränken kann.

## Formalitäten

Die Aufgabe ist in 3er, 4er oder 5er Gruppen zu bearbeiten. Der Fokus ist auf saubere Bearbeitung zu legen.

Die Bewertung wird Anhand folgender Faktoren festgelegt:

- Code Struktur und Qualität
  - Automatische Formatierung mit `cargo fmt`
  - Linting mit `cargo clippy`
  - Modularität
- Effizienz bzgl. Speicher und Laufzeit
- Dokumentation mit `cargo doc` und aussagehähige `README.md`
- Tests mit `cargo test`
- Gruppeninterviews
  - Jedes Gruppenmitglied kann erklären, woran sie/er gearbeitet hat.
  - Pair/Mob Programming ist eine sehr gute Option.
  - Jede Person wird individuell bewertet.

