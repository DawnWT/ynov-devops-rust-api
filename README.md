# Rust Ping API

Cette application lance un serveur http renvoyant les headers de la requete vers /ping

## Installation

- cloner le repo
- ```cargo run```

## Usage

Le port du serveur peut etre modifié via la variables d'environnement `PING_LISTEN_PORT`, via un fichier `.env` ou les variables d'environnements

La route `/ping` renvoit les headers de la requete sous forme de JSON

Toutes autres routes renvoie un `404 NOT FOUND` vide