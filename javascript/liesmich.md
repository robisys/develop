# Javascript
## promise

[promise](http://wiki.selfhtml.org/wiki/JavaScript/Promise)

Ein Promise ist ein Objekt, das asynchrone Operationen in einer einheitlichen API kapselt. Dieses kann verarbeitet werden, bevor die Operation abgeschlossen ist. Es bietet eine Alternative zum mitunter umständlichen Umgang mit Callback-Funktionen.
  
Es gibt unterschiedliche Möglichkeiten, in JavaScript asynchrone Vorgänge auszulösen. Zum Beispiel durch Requests an einen Server (Ajax, Laden von Ressourcen) oder Web Worker. Um die weitere Arbeit mit diesen Vorgängen zu synchronisieren, können Promises genutzt werden.

Der Einsatz eines Promise verläuft in zwei Schritten.

Im ersten Schritt wird ein Promise erzeugt und dem Konstruktor eine Funktion als Parameter übergeben, der so genannte Exekutor. Dieser Exekutor wird vom Konstruktor sofort aufgerufen und bekommt seinerseits zwei Callbackfunktionen als Parameter übergeben, den Erfüller (resolve) und den Zurückweiser (reject). Was der Exekutor genau tut, ist ihm überlassen, ein Promise ist aber nur dann sinnvoll, wenn er irgendeine Form von asynchroner Aktivität startet und dabei so vorgeht, dass dabei sichergestellt wird, dass nach Abschluss dieser Aktivität der Erfüller aufgerufen wird und im Fall eines Fehlers der Zurückweiser.

Syntax

    new Promise(function(resolve, reject) { ... });

Dieses Promise kann nun als Lieferant für das irgendwann in der Zukunft verfügbare Ergebnis der asynchron gestarteten Aktivität verwendet werden. Solange der Exekutor nicht den Erfüller oder Zurückweiser aufgerufen hat, ist das Promise in einem Schwebezustand (pending). Nachdem einer der beiden aufgerufen wurde, ist das Promise festgelegt (settled) und zwar entweder auf erfüllt (fulfilled) oder zurückgewiesen (rejected).

Erfüller und Zurückweiser erwarten jeweils einen Parameter. Der Erfüller den Erfüllungswert des Promise, der Zurückweiser den Zurückweisungsgrund. Diese Werte werden vom Promise später zur Verfügung gestellt.

Das weitere Verhalten des Promise hängt davon ab, in welchem Zustand es ist. Der Nutzer des Promise verwendet zwei Methoden, die auf Promise.prototype definiert sind und die Erfüllungs- bzw. Zurückweisungs-Callbackfunktionen als Parameter erwarten:

      then(onFulfill, onReject)
      catch(onReject)

Die catch-Methode ist eine Kurzform von then(null, onReject). Solange das Promise im Status pending ist, erstellen beide Funktionen lediglich einen Warteschlangeneintrag im Promise. In dem Moment, wo der Exekutor einen seiner beiden Callbacks aufruft, wird der Status des Promise auf fulfilled oder rejected geändert und dann die zugehörige Callback-Warteschlange abgearbeitet. Dabei bekommen die onFullfill-Funktionen den Erfüllungswert des Promise als Parameter, und die onReject-Funktionen den Zurückweisungsgrund.

Wird then oder catch aufgerufen, wenn das Promise bereits settled ist, wird die entsprechende Callbackfunktion direkt aufgerufen.

Genauere Informationen zu then und catch, unter anderem zu ihren Parametern und der Interaktion mit den Erfüller- und Zurückweiserfunktionen, werden Sie auf den entsprechenden Wiki-Seiten finden, sobald sie erstellt sind.

Eine Besonderheit von then und catch ist, dass sie ihrerseits ein Promise zurückgeben. Der Übersicht halber soll das zuvor erzeugte Promise als das ursprüngliche Promise bezeichnet werden, und dieses neue Promise als das "äußere Promise" (denn gleich folgt noch ein inneres). Das äußere Promise ist solange im Status pending, bis die Callbacks von then bzw. catch ausgeführt wurden. Wie es weitergeht, hängt vom Rückgabewert des Callbacks ab:

Ist es ein Wert (auch undefined), der kein Promise ist, wird das äußere Promise auf fulfilled gesetzt und der zurückgegebene Wert wird zum Erfüllungswert. Wichtig: Das gilt auch für den Zurückweisungscallback. Das ursprüngliche Promise kann rejected sein, aber wenn der Zurückweisungscallback beispielsweise 'true' zurückgibt, gilt das äußere Promise als fulfilled.

Wirft der Callback eine Exception, gilt das äußere Promise als rejected. Das geworfene Exceptionobjekt wird als Zurückweisungsgrund gesetzt.

Wird vom Callback ein Promise zurückgegeben (das innere Promise), so wird der Status des äußeren Promise mit dem Status des inneren Promise gekoppelt. Sobald das innere Promise von pending auf einen settled-Zustand wechselt, wechselt das äußere Promise mit.

Das bedeutet, dass man Promise-Ketten bilden kann. Angenommen, die Funktionen doSomethingWithPromise und promiseMeSomethingMore lösen beide asynchrone Aktivitäten aus und liefern ein Promise dazu zurück. Dann kann man so erreichen, dass beide Aktivitäten nacheinander ablaufen und danach eine Abschlussfunktion gerufen wird:

     doSomethingWithPromise()
    .then(promiseMeSomethingMore) 
     .then(completeTheTask)

Wichtig bei then ist noch, dass eine Exception im Erfüllungscallback nicht den Zurückweisungscallback des gleichen then-Aufrufs auslöst. Will man das erreichen, muss man den Umstand nutzen, dass eine Exception in Erfüllungscallback das von then zurückgegebene Promise auf rejected setzt. D.h. mit diesem Konstrukt

    doSomethingWithPromise()
    .then(receiveResult)
    .catch(handleFailure)

kann man in der handleFailure-Funktion sowohl ein zurückgewiesenes Promise als auch eine Exception in receiveResult behandeln. 


[promises](http://www.peterkroener.de/ecmascript-6-promises/)

[Future_(Programmierung)](https://de.wikipedia.org/wiki/Future_(Programmierung))
