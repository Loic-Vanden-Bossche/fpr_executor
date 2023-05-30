FROM fpr-executor-python:latest

COPY sample.py /game/game.py

ENTRYPOINT ["/usr/local/bin/fpr-executor", "--exec-type", "python", "--debug", "--script-path", "/game/game.py"]