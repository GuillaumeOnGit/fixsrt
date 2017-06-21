pub const RULES_FR: &'static [(&'static str, &'static str)] = &[
	// Trop d'espaces
	("  ", " "),

	// Espace insécable
	("+!",  "\u{A0}!"),
	("+ !", "\u{A0}!"),
	("+?",  "\u{A0}?"),
	("+ ?", "\u{A0}?"),

	// Ordinaux
	("1er",   "1ᵉʳ"),
	("1ers",  "1ᵉʳˢ"),
	("1ère",  "1ʳᵉ"),
	("1ères",  "1ʳᵉˢ"),

	("2ème",  "2ᵉ"),
	("3ème",  "3ᵉ"),
	("4ème",  "4ᵉ"),
	("5ème",  "5ᵉ"),
	("6ème",  "6ᵉ"),
	("7ème",  "7ᵉ"),
	("8ème",  "8ᵉ"),
	("9ème",  "9ᵉ"),

	("#0ème",  "0ᵉ"),
	("#1ème",  "1ᵉ"),
	("#2ème",  "2ᵉ"),
	("#3ème",  "3ᵉ"),
	("#4ème",  "4ᵉ"),
	("#5ème",  "5ᵉ"),
	("#6ème",  "6ᵉ"),
	("#7ème",  "7ᵉ"),
	("#8ème",  "8ᵉ"),
	("#9ème",  "9ᵉ"),

	("2e",  "2ᵉ"),
	("3e",  "3ᵉ"),
	("4e",  "4ᵉ"),
	("5e",  "5ᵉ"),
	("6e",  "6ᵉ"),
	("7e",  "7ᵉ"),
	("8e",  "8ᵉ"),
	("9e",  "9ᵉ"),

	("#0e",  "0ᵉ"),
	("#1e",  "1ᵉ"),
	("#2e",  "2ᵉ"),
	("#3e",  "3ᵉ"),
	("#4e",  "4ᵉ"),
	("#5e",  "5ᵉ"),
	("#6e",  "6ᵉ"),
	("#7e",  "7ᵉ"),
	("#8e",  "8ᵉ"),
	("#9e",  "9ᵉ"),

	("2è",  "2ᵉ"),
	("3è",  "3ᵉ"),
	("4è",  "4ᵉ"),
	("5è",  "5ᵉ"),
	("6è",  "6ᵉ"),
	("7è",  "7ᵉ"),
	("8è",  "8ᵉ"),
	("9è",  "9ᵉ"),

	("#0è",  "0ᵉ"),
	("#1è",  "1ᵉ"),
	("#2è",  "2ᵉ"),
	("#3è",  "3ᵉ"),
	("#4è",  "4ᵉ"),
	("#5è",  "5ᵉ"),
	("#6è",  "6ᵉ"),
	("#7è",  "7ᵉ"),
	("#8è",  "8ᵉ"),
	("#9è",  "9ᵉ"),

	// Cédille
	("ca", "ça"),
	("Ca", "Ça"),
	("lecon",   "leçon"),

	("D'ou", "D'où"),

	// Capitales accentuées
	("Ecart*",     "Écart"),
	("Echantillon","Échantillon"),
	("Eclaire",    "Éclaire"),
	("Ecole",      "École"),
	("Economis*",  "Économis*"),
	("Ecout*",     "Écout"),
	("Ecras*",     "Écras"),
	("Ecris*",     "Écris"),
	("Ecume",      "Écume"),
	("Edition",    "Édition"),
	("Egal",       "Égal"),
	("Egalement",  "Également"),
	("Egoïste",    "Égoïste"),
	("Egypte",     "Égypte"),
	("Egyptien*",  "Égyptien"),
	("Elevé",      "Élevé"),
	("Eloign*",    "Éloign"),
	("Etaient",    "Étaient"),
	("Etais",      "Étais"),
	("Etant",      "Étant"),
	("Etat",       "État"),
	("Eteins ça",  "Éteins ça"),
	("Eteins-moi", "Éteins-moi"),
	("Eteins le",  "Éteins le"),
	("Eteins la",  "Éteins la"),
	("Etiez-vous", "Étiez-vous"),
	("Etonnant",   "Étonnant"),
	("Evidemment", "Évidemment"),
	("Evite*",     "Évite"),

	("Etes", "Êtes"),
	("Etre", "Être"),
	
	("A 1*",        "À 1"),
	("A 2*",        "À 2"),
	("A 3*",        "À 3"),
	("A 4*",        "À 4"),
	("A 5*",        "À 5"),
	("A 6*",        "À 6"),
	("A 7*",        "À 7"),
	("A 8*",        "À 8"),
	("A 9*",        "À 9"),
	("A bientôt",   "À bientôt"),
	("A cause",     "À cause"),
	("A ceci",      "À ceci"),
	("A ce",        "À ce"),
	("A chaque",    "À chaque"),
	("A combien",   "À combien"),
	("A commencer", "À commencer"),
	("A condition", "À condition"),
	("A croire",    "À croire"),
	("A courir",    "À courir"),
	("A demain",    "À demain"),
	("A déplacer",  "À déplacer"),
	("A des",       "À des"),
	("A deux",      "À deux"),
	("A emporter",  "À emporter"),
	("A faire",    "À faire"),
	("A fumer",    "À fumer"),
	("A l'*",      "À l'"),
	("A ma",       "À ma"),
	("A me",       "À me"),
	("A moins",    "À moins"),
	("A mon",      "À mon"),
	("A genoux",   "À genoux"),
	("A la",       "À la"),
	("A Los",      "À Los"),
	("A ne",       "À ne"),
	("A notre",    "À notre"),
	("A nous",     "À nous"),
	("A nouveau",  "À nouveau"),
	("A part",     "À part"),
	("A peine,",   "À peine,"),
	("A peine",    "À peine"),
	("A peu près", "À peu près"),
	("A plus",     "À plus"),
	("A partir",   "À partir"),
	("A présent",  "À présent"),
	("A propos",   "À propos"),
	("A quelle",   "À quelle"),
	("A quoi",     "À quoi"),
	("A rien",     "À rien"),
	("A son",      "À son"),
	("A t'*",      "À t'"),
	("A table",    "À table"),
	("A terre",    "À terre"),
	("A te",       "À te"),
	("A tes",      "À tes"),
	("A toi",      "À toi"),
	("A ton",      "À ton"),
	("A tou",      "À tou"),
	("A un",       "À un"),
	("A votre",    "À votre"),
	("A vous",     "À vous"),
	("A vos",      "À vos"),
	("A vrai dire", "À vrai dire"),
	("A vendredi",  "À vendredi"),
	
	// Ligature œ
	("boeuf",     "bœuf"),
	("Coeur",     "Cœur"),
	("coeur",     "cœur"),
	("coeurs",    "cœurs"),
	("foetus",    "fœtus"),
	("oeuf",      "œuf"),
	("oeufs",     "œufs"),
	("oei*",      "œi"),
	("Oeuf",      "Œuf"),
	("Oeufs",     "Œufs"),
	("Oei*",      "Œi"),
	("oestrogène","œstrogène"),
	("manoeuvrer","manœuvrer"),
	("noeud",     "nœud"),
	("noeuds",    "nœuds"),
	("recue",     "reçue"),
	("recus",     "reçus"),
	("soeur",     "sœur"),
	("soeurs",    "sœurs"),
	("oeuvre",    "œuvre"),
	("oeuvres",   "œuvres"),
	("Oeuvre",    "Œuvre"),
	("Oeuvres",   "Œuvres"),
	("voeux",     "vœux"),

	// Misc
	("des qu'*", "dès qu'"),
	("que tu ais", "que tu aies"),
	("Tous les 2", "Tous les deux"),
	("J'ai du vérifier", "J'ai dû vérifier"),
	("c'est règlé", "c'est réglé"),
	("Quelque soient", "Quels que soient"),

	// Sûr
	("Je suis sur qu'*", "Je suis sûr qu'"),
	("Bien sur,", "Bien sûr,"),
	("bien sur,", "bien sûr,"),
	
	// Subjonctif présent 2e personne
	("N'ais", "N'aies"),

	// Participe passé au lieu d'infinitif
	(" se déplacé", " se déplacer"),
	
	// Conjuguaison
	("Je doit", "Je dois")
];
