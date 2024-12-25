<h1>RUN APP :</h1>

    in prod :
        export DOCKER_BUILD_TARGET=prod
        docker compose up --build -d

    in dev : 
        export DOCKER_BUILD_TARGET=dev
        docker compose up --build