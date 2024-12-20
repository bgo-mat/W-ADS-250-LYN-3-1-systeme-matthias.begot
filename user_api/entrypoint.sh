#!/bin/sh
set -e
diesel migration run
echo "Migrations appliquées. Lancement de l'application..."
user_api
echo "user_api s'est terminé."
