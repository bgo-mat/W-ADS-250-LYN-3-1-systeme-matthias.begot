#!/bin/sh
set -e
echo "Attente de la base de données..."
until mysqladmin ping -h db --silent; do
  sleep 2
done
echo "Base de données disponible, lancement des migrations..."
diesel migration run
echo "Migrations appliquées. Lancement de l'application..."
exec user_api
