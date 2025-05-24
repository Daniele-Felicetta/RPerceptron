INPUT:
  - Dati di addestramento: (X, y), con N esempi
      X = [x¹, x², ..., xᴺ]  ← ogni xⁱ è un vettore di input
      y = [y¹, y², ..., yᴺ]  ← ogni yⁱ ∈ {−1, +1}
  - Tasso di apprendimento: η
  - Numero massimo di epoche: max_epochs

INIZIALIZZAZIONE:
  - Inizializza i pesi w (uno per ogni input, spesso a 0 o valori piccoli casuali)
  - Inizializza bias b = 0

ALGORITMO:
  Per epoca da 1 a max_epochs:
    errore_totale = 0

    Per ogni esempio (xⁱ, yⁱ) in (X, y):
        1. Calcola output:
           z = w · xⁱ + b
           y_pred = segno(z)

        2. Calcola errore:
           errore = yⁱ − y_pred

        3. Se errore ≠ 0:  // cioè se il perceptron ha sbagliato
           - Aggiorna i pesi:
               w = w + η × errore × xⁱ
           - Aggiorna il bias:
               b = b + η × errore
           - errore_totale += 1

    Se errore_totale == 0:
        Ferma l’addestramento (il modello ha imparato)

USCITA:
  - Restituisci i pesi w e il bias b