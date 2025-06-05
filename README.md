# Perceptron e Classificazione Lineare

**Rosemblatt** propone un algoritmo in grado di apprendere automaticamente *i coefficenti dei pesi* ottimali  

- Vengono perciò utilizzati dei neuroni artificiali per un compito di **classificazione binario:**
    - Classe positiva = 1
    - Classe negativa = -1
    
- La predizione del modello è data dalla **funzione decisionale $\phi(z)$**
    
     $output =
    \begin{cases}
     1 = z >= 0 \\
     -1 = z < 0   
    \end{cases}$
    
    Dove **$z$** è una ****combinazione lineare e** input della rete $z = w_i x_i$ + bias
    

Se per esempio $x_i$ è maggiore di una soglia $0$ possiamo **predire la classe positiva e quella negativa**

### Passi nell’algoritmo Perceptron di Rosemblatt

1. Inizializzare pesi a $0$ o a un numero random
2. Per ogni  $x_i$:
    - calcolare output
    - aggiornare i pesi

L’output è l’etichetta della classe che è stata predetta dalla ***funzione a passo unitario*** ( **funzione decisionale $\phi(z)$)** 

### Aggiornamento dei pesi

E quindi il conseguente aggiornamento dei pesi $w$:

$$
w_j := w_j + \Delta w_j
$$

[Aggiornamento dei pesi](https://www.notion.so/Aggiornamento-dei-pesi-1fabc6e1df02807793efd7c8d206969b?pvs=21)

### Aggiornamento dei bias

$$
b:=b+η(y−ŷ)
$$

[**Iperpiano come “parete” tra due classi**](https://www.notion.so/Iperpiano-come-parete-tra-due-classi-1ffbc6e1df02807eb8cfe7d4beb4c804?pvs=21)

## Pseudocodice del Perceptron

INPUT:

- Dati di addestramento: (X, y), con N esempi
X = [x¹, x², ..., xᴺ] ← ogni xⁱ è un vettore di input
y = [y¹, y², ..., yᴺ] ← ogni yⁱ ∈ {−1, +1}
- Tasso di apprendimento: η
- Numero massimo di epoche: max_epochs

INIZIALIZZAZIONE:

- Inizializza i pesi w (uno per ogni input, spesso a 0 o valori piccoli casuali)
- Inizializza bias b = 0

ALGORITMO:
Per epoca da 1 a max_epochs:
errore_totale = 0

```
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

```

USCITA:

- Restituisci i pesi w e il bias b
