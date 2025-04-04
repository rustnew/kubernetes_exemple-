# **Projet d'Interaction avec Kubernetes en Rust**

Ce projet démontre comment utiliser les bibliothèques Rust pour interagir avec Kubernetes en créant et gérant un pod Nginx. Voici une analyse complète :

## **Description du Projet**
Un opérateur Kubernetes minimal qui :
1. Se connecte à un cluster Kubernetes
2. Crée un pod Nginx avec un port exposé
3. Surveille l'état du pod jusqu'à ce qu'il soit en état "Running"
4. Récupère et affiche les logs du pod

## **Dépendances Clés Utilisées**
| Bibliothèque | Utilité |
|-------------|---------|
| `kube` | Client principal pour Kubernetes |
| `k8s-openapi` | Types pour les ressources Kubernetes |
| `tokio` | Runtime asynchrone |
| `futures` | Traitement des streams asynchrones |

## **Explication du Code**

### **1. Initialisation du Client Kubernetes**
```rust
let client = Client::try_default().await?;
let pods: Api<Pod> = Api::default_namespaced(client);
```
- Se connecte au cluster en utilisant le contexte par défaut (~/.kube/config)
- Crée une API client ciblant les pods dans le namespace par défaut

### **2. Définition du Pod**
```rust
let pod = Pod {
    metadata: ObjectMeta {
        name: Some("nginx-example".to_string()),
        ..Default::default()
    },
    spec: Some(PodSpec {
        containers: vec![Container {
            name: "nginx".to_string(),
            image: Some("nginx:latest".to_string()),
            ports: Some(vec![ContainerPort {
                container_port: 80,
                ..Default::default()
            }]),
            ..Default::default()
        }],
        ..Default::default()
    }),
    ..Default::default()
};
```
- Configure un pod avec :
  - Un seul container Nginx
  - Port 80 exposé
  - Nom "nginx-example"

### **3. Création du Pod**
```rust
let pod = pods.create(&PostParams::default(), &pod).await?;
println!("Created pod: {}", pod.name_any());
```
- Envoie la définition du pod à l'API Kubernetes
- Affiche le nom du pod créé

### **4. Attente du Statut "Running"**
```rust
loop {
    let pod = pods.get("nginx-example").await?;
    let status = pod.status.as_ref().expect("Pod status should be available");
    
    if let Some(phase) = &status.phase {
        if phase == "Running" {
            println!("Pod is running");
            break;
        }
    }
    
    sleep(Duration::from_secs(1)).await;
}
```
- Vérifie périodiquement l'état du pod
- Sort de la boucle quand le pod est en état "Running"

### **5. Récupération des Logs**
```rust
let logs = pods.logs("nginx-example", &Default::default()).await?;
println!("Pod logs:\n{}", logs);
```
- Récupère les logs stdout/stderr du container
- Les affiche dans la console

## **Comment Lancer le Projet**

### **Prérequis**
1. Cluster Kubernetes actif (Minikube, Kind, ou cloud)
2. `kubectl` configuré avec accès au cluster
3. Rust et Cargo installés

### **Étapes**
1. Cloner le projet :
   ```bash
   git clone <votre-projet>
   cd <votre-projet>
   ```

2. Construire le projet :
   ```bash
   cargo build
   ```

3. Exécuter :
   ```bash
   cargo run
   ```

4. Vérifier dans Kubernetes :
   ```bash
   kubectl get pods
   kubectl logs nginx-example
   ```

## **Extensions Possibles**
1. **Gestion des Erreurs** : Ajouter `thiserror` pour des erreurs typées
2. **Watch Events** : Utiliser `WatchEvent` pour surveiller les changements
3. **Custom Resources** : Implémenter un CRD avec `schemars`
4. **Tests** : Utiliser `kube::Client::mock` pour des tests unitaires

## **Bonnes Pratiques**
- Toujours vérifier les `Option` et `Result`
- Limiter les permissions RBAC au strict nécessaire
- Utiliser des timeouts explicites
- Implémenter du logging structuré avec `tracing`

Ce projet sert de base pour développer des opérateurs Kubernetes plus complexes en Rust, combinant sécurité et performances.
