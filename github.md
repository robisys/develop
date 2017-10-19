http://t3n.de/news/versionskontrolle-git-github-598054/


Mit Hilfe von Versionskontroll-Systemen kannst du dir und deinem Team die Arbeit an Web-Projekten deutlich vereinfachen. Aber auch für Solo-Projekte kann eine gute Versionierung sehr nützlich sein.

Wenn es um das Thema Versionierung geht, bekommt man von Laien oft zu hören, dass es ein unnötiger Aufwand sei, Projekte lokal, auf den jeweiligen Servern und dann auch noch in einer Versionskontrolle aktuell zu halten und dass der direkte Weg über den FTP-Client doch deutlich einfacher und effizienter sei. Das mag auf den ersten Blick tatsächlich so erscheinen. Hat man jedoch erst mal mit einem Versionskontroll-System wie GIT gearbeitet, wird schnell klar,
dass die Vorteile in Relation zum zusätzlichen Aufwand überwiegen.

Versionskontroll-Systeme helfen dabei, einen Überblick über deine Web-Projekt zu behalten. Das System behält für dich sämtliche Dateien im Auge und legt Kopien von aktualisierten Dokumenten an. So hast du die Möglichkeit, jederzeit zu sehen, welche Dateien wann geändert wurden, und kannst die unterschiedlichen Versionen der Dateien schnell vergleichen, zusammenführen und Änderungen rückgängig machen. Doch wie funktionieren Versionskontroll-Systeme?
Eins der wohl bekanntesten Systeme ist GIT mit der dazugehörigen Online-Plattform GitHub, das seinen Usern unter anderem ein auf GIT basierendes Versionskontroll-System anbietet. Auf GitHub werden hierfür Projekte in so genannten Repositorys organisiert. Diese „Archive“ beinhalten die jeweiligen Bestandteile wie PHP-, JavaScript-, HTML- und CSS-Dateien.

Am Projekt beteiligte Benutzer können eine Kopie des Repositorys auf ihrem Computer anlegen und daran arbeiten. Durchgeführte Änderungen schicken sie an GitHub zurück, das dann den Rest der Arbeit wie das Anlegen von Sicherungskopien der alten Dateien übernimmt.
Das Versionskontroll-System GIT nutzen

Um von einer zentralen Versionskontrolle profitieren zu können, musst du zunächst eine lokale Versionskontrolle auf deinem Computer installieren. GIT ist für alle Plattformen verfügbar und ist mit wenigen Klicks einsatzbereit. Auf der offiziellen Webseite zum Versionskontrollsystem GIT kannst du das jeweils passende Installationspaket runterladen.

In diesem Artikel werden wir mit GitHub arbeiten. Damit du loslegen kannst und das System deine Änderungen auch entsprechend erfassen kann, musst du neben dem Repository auf GitHub auch ein lokales Repository anlegen. Das geht beispielsweise mit dem GitHub-Client (GitHub für Mac, GitHub für Windows). Er kann ein bestehendes Repository von einem zentralen in dein lokales Versionskontroll-System kopieren (Cloning). Du hast auch die Möglichkeit, ein Repository zunächst lokal anzulegen und dann auf GitHub zu veröffentlichen. Wir gehen aber den einfachen Weg. Ist die Kopie des Repositorys erst mal angelegt, kannst du wie gewohnt mit dem Editor deiner Wahl an dem Projekt weiter arbeiten.
GitHub für MacVersionskontrolle: Mit dem GitHub-Client kannst du ganz einfach bei GitHub angelegte Projekte auf deinen Rechner kopieren. (Screenshot: t3n)
Commit
GitHub für MacMit einem Commit speicherst du eine neue Version deiner Dateien im lokalen Repository. (Screenshot: t3n)

Sobald du mit deiner Arbeit fertig bist, musst du deine Änderungen für das Versionskontroll-System zusammenfassen. Das geht mit einem so genannten „Commit“. Mit dem Commit-Befehl teilst du deinem lokalen Repository mit, dass du bestimmte Dateien verändert hast und dieser Schritt jetzt in die Versionierung übernommen werden soll. Hierfür kannst du die Dateien, die du verändert hast, auswählen und einen Kommentar hinterlassen, um später besser nachvollziehen zu können, was du gemacht hast.
Push
GitHub Remote PublishMit einem Push (GitHub: Publish) veröffentlichst du Änderungen deines lokalen Repositorys im zentralen Repository. (Screenshot: t3n)

Wenn du deine Änderungen an die zentrale Versionsverwaltung – also GitHub – übermitteln willst, musst du die Änderungen pushen.Beim Pushen werden sämtliche Änderungen aus deinem lokalen Repository an das Remote-Repository übertragen. Dazu gehören auch Commits, die du zwischendurch durchgeführt hast. Beim GitHub-Client nennt sich dieser Schritt „Publish“. So können auch offline durchgeführte Änderungen erfasst und später im Remote-Repository nachvollzogen werden.
Pull

Der Pull-Befehl ist das Gegenteil vom Push: Mit einem Pull (beim GitHub-Client „Sync“) kannst du dein lokales Repository mit Daten aus dem entfernten Repository abgleichen und Änderungen übernehmen. Das kann besonders nützlich werden, wenn du an mehreren Geräten arbeitest, da so alle deine Entwicklungsgeräte die aktuellste Fassung des Projektes zur Verfügung haben. Natürlich könnte man auch ohne GIT die anderen Entwicklungsgeräte synchron halten – beispielsweise über FTP. Das dauert aber mit zunehmender Projektgröße deutlich länger als das Abgleichen deiner Dateien mit einem Repository.
Branching
GitHub BranchMit einem Branch erstellst du eine Kopie des Repositorys oder eines anderen Branches und kannst parallel entwickeln. (Screenshot: t3n)

Ein GIT-Projekt kann mit so genannten Branches aufgeteilt werden. Sobald du oder ein anderer Entwickler einen Branch anlegt, wird eine eine 1:1-Kopie des aktuellen Zustands (der so genannte Stamm) angelegt, an der dann parallel weitergearbeitet werden kann.

Das ist beispielsweise sinnvoll, wenn du am Abrechnungsmodul deines Projekts arbeitest, während ein anderer Entwickler sich um das Projekt-Management-Modul kümmert und ein dritter die Core-Funktionen noch mal auf Fehler überprüft. So kann von jedem der drei Module eine aktuelle Version gepflegt werden, die zu einem späteren Zeitpunkt in den Stamm-Code einfließen kann.
Merging
GitHub MergingDas Merging mit dem GitHub-Client ist mit wenigen Klicks erledigt. (Screenshot: t3n)

Als Merging bezeichnet man die Zusammenfassung von mehreren Branches. Hierfür werden die Änderungen an einem Branch in einen anderen Branch (oder Stamm) übernommen. Das funktioniert besonders gut, wenn der Branch Änderungen an Dateien vornimmt, die von keinem anderen Benutzer in dem Branch, mit dem die Zusammenführung durchgeführt werden soll, bearbeitet wurden. Im GitHub-Client werden Branches ganz einfach per Drag & Drop in der „Branches-Ansicht“ zusammengeführt.
Vor- und Nachteile von Systemen zur Versionskontrolle

Für die Zusammenarbeit mit einem Versionskontroll-System wie GIT ist eine gewisse Disziplin erforderlich. Zwar hat GIT eine Möglichkeit, Änderungen verschiedener Programmierer an der selben Datei zusammenzuführen. Wirklich unproblematisch ist das aber nur in seltenen Fällen. Es ist somit sinnvoll, jemanden zu bestimmen, der die Arbeit an den einzelnen Komponenten koordiniert und somit Kollisionen frühzeitig verhindern kann. Ein modularer Aufbau des Projekts begünstigt die Zusammenarbeit mit einem Versionskontroll-System natürlich.

Die hier beschriebenen Möglichkeiten kratzen zudem nur an der Oberfläche dessen, was mit einem Versionskontroll-System möglich ist. Für den Einstieg und ein erstes gemeinsames Projekt sollte das aber reichen. Doch Versionskontroll-Systeme sind nicht nur für Teams interessant. Auch wenn du alleine arbeitest, kann der Einsatz eines solchen Systems dir die Arbeit vereinfachen – sei es durch eine Sicherung oder die konsequente Trennung und Versionierung von Features in unterschiedlichen Branches.
Nicht nur für Webworker

Versionskontroll-Systeme sind aber nicht nur für Programmierer interessant. Zwar ist ein System wie GIT nicht für die Verwaltung und das Zusammenlegen von Binärdaten ausgelegt, das bedeutet aber nicht, dass es nicht dazu in der Lage ist, Sicherungskopien älterer Versionen von beispielsweise Photoshop- oder InDesign-Dateien anzulegen. So könnte ein solches System auch für die Versionierung von Layouts verwendet werden, ohne hierfür unzählige inkrementelle Kopien anfertigen zu müssen. Irgendwann passiert es schließlich jedem mal, dass man die falsche Datei überschreibt. Mit einem Versionskontroll-System wie GIT ist das dann kein Problem mehr.
Kostenlose öffentliche und private Repositorys erstellen

GiHub bietet seinen Nutzern eine Möglichkeit, kostenlose Repositorys anzulegen – die sind jedoch öffentlich und für alle einsehbar. Wenn du von einem zentralen Versionskotroll-System profitieren willst, aber die Kosten von GitHub scheust, könnte sich ein Blick auf BitBucket lohnen. Der Dienst von Atlassian bietet ebenfalls zentrale GIT-Repositorys an, die aber auch schon im kostenlosen Tarif mit bis zu fünf Team-Mitgliedern privat geteilt werden können. 
