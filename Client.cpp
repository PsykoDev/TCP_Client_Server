//
//  main.cpp
//  TCPClientCPP
//
//  Created by psyko on 16/10/2023.
//

#include <iostream>
#include <cstring>
#include <unistd.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <sys/select.h>
#include <fcntl.h>

int main() {
    int clientSocket = socket(AF_INET, SOCK_STREAM, 0);
    if (clientSocket == -1) {
        std::cerr << "Erreur lors de la création du socket." << std::endl;
        return 1;
    }

    struct sockaddr_in serverAddr;
    serverAddr.sin_family = AF_INET;
    serverAddr.sin_port = htons(1234);
    serverAddr.sin_addr.s_addr = inet_addr("127.0.0.1");

    if (connect(clientSocket, (struct sockaddr*)&serverAddr, sizeof(serverAddr)) == -1) {
        std::cerr << "Erreur lors de la connexion au serveur." << std::endl;
        return 1;
    }

    int flags = fcntl(clientSocket, F_GETFL, 0);
    fcntl(clientSocket, F_SETFL, flags | O_NONBLOCK);

    std::cout << "Bienvenue sur le client de chat multicanal!" << std::endl;
    std::cout << "Entrez votre nom d'utilisateur : ";
    std::string username;
    std::getline(std::cin, username);
    username += "\n";
    
    send(clientSocket, username.c_str(), username.size(), 0);

    fd_set read_fds;
    FD_ZERO(&read_fds);
    FD_SET(STDIN_FILENO, &read_fds);
    FD_SET(clientSocket, &read_fds);

    char message[4096];
    std::string fullMessage;

    while (true) {
        fd_set tmp_fds = read_fds;
        if (select(clientSocket + 1, &tmp_fds, NULL, NULL, NULL) < 0) {
            std::cerr << "Erreur lors de l'appel à select." << std::endl;
            break;
        }

        if (FD_ISSET(STDIN_FILENO, &tmp_fds)) {
            std::getline(std::cin, fullMessage);
            if (fullMessage.empty()) {
                //close(clientSocket);
                //break;
            }

            send(clientSocket, fullMessage.c_str(), fullMessage.size(), 0);
        }

        if (FD_ISSET(clientSocket, &tmp_fds)) {
            int bytesRead = recv(clientSocket, message, sizeof(message), 0);
            if (bytesRead <= 0) {
                std::cerr << "La connexion a été fermée par le serveur." << std::endl;
                close(clientSocket);
                break;
            }
            message[bytesRead] = '\0';
            std::cout << message << std::endl;
        }
    }

    return 0;
}








