# Nats with Rust
At the core of MSA will be interactions between micro services.<br>
Although simple point-to-point interaction may be enough for certain use cases,<br>
MSA inherently involves increased global complexity in exchange for reduced local complexity.<br>
In this respect, introducing queue service such as Nats or Kafka will benefit the entire system<br>
as it decouples system components while providing reliability and resiliency against failure.<br><br>

In this project, I examplified simple use cases of Nats jet-stream.