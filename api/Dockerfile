FROM golang:1.22.6

WORKDIR /app

COPY go.mod .
RUN go mod download && go mod verify

COPY . .
RUN go build -v -o main .

CMD ["/app/main"]
