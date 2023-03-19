# WebShell
Hand made ssh daemon for linux.

## :bulb: The idea 
Create a study program that receives commands over the network, execute them and sends the answer as [openSSH](https://en.wikipedia.org/w/index.php?title=OpenSSH).

## :point_up: The reason
- Learn how to work with a docker
- Understand linux sockets
- Create a daemon
- Practise parsing and multitasking

## :ok_hand: Implementation
1.  Create a docker container.
2. Create daemon:
   - Listen network requests with cmd.
   - Create thread to execute.
   - Parse bash like cmd.
   - Sent output as response.
3. Set up a systemd daemon.
4. Create auto setup script and docket-compose file.
5. Enjoy.
