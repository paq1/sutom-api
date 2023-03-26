# sutom-api

## description
Cette api fait l'interface entre une db mongo permettant :
<ul>
    <li>d'avoir un historique de toutes nos partie par joueur</li>
    <li>avoir un systeme de score  (wip)</li>
    <li>avoir un systeme de classement (wip)</li>
</ul>
Cette api sera consommée par un de mes projets: <a href="https://github.com/paq1/sutombot">sutombot</a> <br>
C'est mon bot discord qui permettra la communication avec cette API.
NB: je vais bientôt mettre en place un front d'admin de l'api en Rust avec le framework Dioxus 😊

## deploiement

### image docker 
build de l'image :
```
docker build -t <nom-image> .
```
connection a docker :
```
docker login
```
```
docker tag my_app <username>/my_app:latest
docker push <username>/my_app:latest
```
### image docker 
lancement des différents services
```
docker-compose up
```