# Micat Development Plan

## Objective
Micat will be a fully featured dual server management service capable of handling two Minecraft servers simultaneously: one permanent and one seasonal/temporary. The service will provide robust management tools, automation, and integration capabilities to streamline server operations.

## Core Features

### 1. Dual Server Management
- Manage two servers at once:
  - **Permanent Server**: Always active and stable.
  - **Seasonal/Temporary Server**: For events or temporary use.

### 2. Modpack Integration
- Search, scan, and download files from major modpack hosts (e.g., Modrinth, CurseForge).
- Automate modpack updates and installations.

### 3. User Ownership
- Restrict access to a single user who will have full control over the `/gameserver/` directory and its contents.

### 4. Performance Monitoring
- Track performance metrics for both servers.
- Provide real-time data to a web interface via a Docker container sharing the same socket as the servers.

### 5. Docker Integration
- Utilize Docker and Docker Compose to:
  - Set up and manage server instances.
  - Configure environment variables for each server.

### 6. Backup and Maintenance
- Automate world backups.
- Ensure stable upkeep and recovery options for both servers.

### 7. GUI and CLI Interface
- Develop a locally-hosted graphical user interface (GUI) for managing the service.
- Maintain comprehensive command-line interface (CLI) functionality for advanced users and automation.

### 8. Prebuilt Docker Image for Minecraft Servers
- Use the `itzg/minecraft-server` Docker image for managing both the permanent and seasonal servers.
- Benefits:
  - Supports Forge and NeoForge.
  - Easy setup with minimal configuration.
  - Automatic updates and modpack support.
- Example usage:
  ```bash
  docker run -d -e TYPE=FORGE -e VERSION=1.20.1 -p 25565:25565 -v /path/to/data:/data itzg/minecraft-server
  ```
- Ensure Docker and Docker Compose are installed and configured on the host system.

### 9. Auto-Update and Shutdown Management
- Implement an auto-update mechanism that runs every three days.
- Handle in-game and online broadcasts to warn users of impending shutdowns.
- Ensure a safe shutdown procedure to prevent data loss.

### 10. Always-On Availability
- Ensure the program runs continuously and is always available for input.
- Host the management web application persistently for real-time access.

## Implementation Steps

### Phase 1: Planning and Setup
1. Define detailed requirements for each feature.
2. Set up the development environment.
3. Create a modular architecture to support future expansions.

### Phase 2: Core Development
1. Implement dual server management logic.
2. Integrate APIs for modpack hosts.
3. Develop Docker Compose configurations for server setup.

### Phase 3: Monitoring and Web Integration
1. Build performance monitoring tools.
2. Create a web interface for real-time data visualization.
3. Establish communication between Docker containers.

### Phase 4: Backup and Maintenance
1. Automate backup processes.
2. Implement recovery mechanisms.

### Phase 5: Testing and Deployment
1. Test all features thoroughly.
2. Deploy the service in a controlled environment.
3. Document the setup and usage instructions.

## Future Features
- Expand support for additional server types.
- Add multi-user management capabilities.
- Integrate with third-party tools for enhanced functionality.

## Notes
- Focus on modularity and scalability.
- Ensure security and stability for all components.

# ADDITIONAL FEATURES TO BE ADDED INTO PLAN - KEEP THIS HEADER AND DELETE ENTRIES BELOW THAT HAVE BEEN INCLUDED INTO THE PLAN
