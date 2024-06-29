## Twitter feed back end
Use docker-compose and any programming language

1. Implement an endpoint to add message
2. Implement an endpoint to get feed (get existing messages and stream new ones - use HTTP streaming)
3. Implement back pressure for message creation (use RabbitMQ/Kafka)
4. Use Cockroachdb(at least 2-node cluster) as a database
5. Implement a bot to generate messages (at configurable speed)
**CRITICAL** - Project must start with one command (bash file) without installing anything except docker


docker run -d 
  --env COCKROACH_DATABASE=twitter 
  --env COCKROACH_USER=roach 
  --env COCKROACH_PASSWORD=roach 
  --name=roach-single 
  --hostname=roach-single 
  -p 26257:26257 
  -p 8080:8080 
  -v "roach-single:/cockroach/cockroach-data"  
 cockroachdb/cockroach:latest start-single-node 
  --http-addr=roach-single:8080