# CertificaSol

CertificaSol es mi proyecto final para la Solana Developer Certification de WayLearn. Es un programa escrito en Rust que corre directamente en la blockchain de Solana y permite emitir certificados digitales de forma descentralizada,  sin intermediarios, sin servidores, solo on-chain.

La idea surgió de algo muy sencillo: si estoy aprendiendo a desarrollar en blockchain, ¿por qué no usar esa misma tecnología para registrar el aprendizaje?

---

## Qué hace el programa

CertificaSol permite a cualquier wallet emitir certificados digitales y guardarlos en la blockchain. Cada certificado se almacena en una cuenta PDA (Program Derived Address), lo que significa que su dirección es única, predecible y verificable por cualquier persona.

Las operaciones disponibles son:

- Emitir un certificado nuevo, con título, destinatario y descripción
- Consultar los datos de un certificado existente usando su dirección
- Actualizar el título o descripción de un certificado ya emitido
- Revocar un certificado, eliminando la cuenta y recuperando el SOL depositado

---

## Cómo está estructurado

Cada certificado guarda la siguiente información:

```rust
pub struct Certificado {
    pub emisor: Pubkey,        // Wallet que emitió el certificado
    pub destinatario: String,  // Nombre del graduado o destinatario
    pub titulo: String,        // Nombre del certificado
    pub descripcion: String,   // Descripción del logro o curso
    pub fecha: i64,            // Fecha de emisión en timestamp
    pub bump: u8,              // Dato interno de la PDA
}
```

La dirección de cada certificado se deriva a partir de tres elementos:

```
seeds = ["certificado", wallet_del_emisor, titulo]
```

Esto garantiza que no puedan existir dos certificados con el mismo título emitidos por la misma wallet.

---

## Cómo usarlo

1. Conectar una wallet de Solana en devnet
2. Llamar la instrucción `emitir_certificado` con el título, destinatario y descripción
3. El certificado queda registrado on-chain de forma inmediata
4. Solo la wallet emisora puede actualizarlo o revocarlo

---

## Tecnologías

- Rust
- Anchor Framework
- Solana Devnet
- Solana Playground

---

## Autor

Carlos Andrés de Loera Tarango  
Estudiante de Ingeniería en Ciencia de Datos, Universidad Aurora, Aguascalientes  
Proyecto desarrollado como parte de la Solana Developer Certification — WayLearn 2026
