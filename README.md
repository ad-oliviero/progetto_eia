# Progetto per l'esame di Elementi di Intelligenza Artificiale

## Documentazione
La documentazione è contenuta nella directory [docs/](https://github.com/ad-oliviero/progetto_eia/tree/main/docs)

## Dataset
### Scaricamento
#### Manuale
Per scaricare i dataset, è possibile recarsi alle reciproche pagine sul [sito dell'univesità di Stanford](https://snap.stanford.edu/data)
#### Automatico
Nella cartella del progetto è fornito uno script bash che effettuerà automaticamente il download:
```sh
$ chmod +x ./download-datasets.sh
$ ./download-datasets.sh
```
Lo script utilizza `wget` ed è scritto per sistemi UNIX & UNIX-like

### Dataset Utilizzati
|Nome|Nodi|Archi|Tipo|Dimensione|
|----|----|-----|----|----------|
|[soc-sign-bitcoin-alpha](https://snap.stanford.edu/data/soc-sign-bitcoin-alpha.html)|3783|24186|Labled|152KB|
|[email-Enron](https://snap.stanford.edu/data/email-Enron.html)|36692|183831|Undirected|1.1MB|
|[com-Youtube](https://snap.stanford.edu/data/com-Youtube.html)|1134890|2987624|Undirected|11MB|
|[roadNet-CA](https://snap.stanford.edu/data/roadNet-CA.html)|1965206|2766607|Directed|18MB|
|[as-Skitter](https://snap.stanford.edu/data/as-Skitter.html)|1696415|11095298|Undirected|33MB|
|[cit-Patents](https://snap.stanford.edu/data/cit-Patents.html)|3774768|16518948|Directed|85MB|
|[com-LiveJournal](https://snap.stanford.edu/data/com-LiveJournal.html)|3997962|34681189|Undirected|124MB|

#### Informazioni aggiuntive
- I dataset contengono alcune informazioni nelle prime righe
- I dataset sono in formato txt e le proprie righe sono formate da due numeri:
  - Nodo Sinistro
  - Nodo Destro
- Il dataset [soc-sign-bitcoin-alpha](https://snap.stanford.edu/data/soc-sign-bitcoin-alpha.html) è invence in formato csv. Le sue colonne sono (in ordine da sinistra a destra):
  - **SOURCE**: id del nodo Sinistro
  - **TARGET**: id del nodo Destro
  - **RATING**: il costo delle azioni
  - **TIME**: non rilevante
#### Bibliografia
Eventuali citazioni e Bibliografia, sono contenute nella documentazione [LaTeX](https://github.com/ad-oliviero/progetto_eia/tree/main/docs/documentazione.pdf)
