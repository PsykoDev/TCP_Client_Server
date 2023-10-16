//
//  main.cpp
//  TCPClientCPP
//
//  Created by psyko on 16/10/2023.
//

#include <iostream>
#include <string>
#include <thread>
#include <sys/socket.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <format>

int main() {
    int clientSocket = socket(AF_INET, SOCK_STREAM, 0);
    if (clientSocket == -1) {
        std::cerr << "Failed to create socket" << std::endl;
        return -1;
    }

    struct sockaddr_in serverAddress;
    serverAddress.sin_family = AF_INET;
    serverAddress.sin_port = htons(1234);
    serverAddress.sin_addr.s_addr = inet_addr("127.0.0.1");

    if (connect(clientSocket, (struct sockaddr*)&serverAddress, sizeof(serverAddress)) == -1) {
        std::cerr << "Failed to connect to the server" << std::endl;
        close(clientSocket);
        return -1;
    }

    std::string username;
    std::cout << "Enter your username: ";
    std::getline(std::cin, username);
    
    std::thread receiveThread([clientSocket] {
        char buffer[1024];
        while (true) {
            int bytesRead = recv(clientSocket, buffer, sizeof(buffer), 0);
            if (bytesRead > 0) {
                buffer[bytesRead] = '\0';
                std::cout << "Server: " << buffer;
            } else {
                break;
            }
        }
    });

    std::string message;
    
    while (1) {
        std::getline(std::cin, message);
        message = username + ": " + message + "\n";
        send(clientSocket, message.c_str(), message.length(), 0);
    }

    receiveThread.join();
    close(clientSocket);

    return 0;
}
