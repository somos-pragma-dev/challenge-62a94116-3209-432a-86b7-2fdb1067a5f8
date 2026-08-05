# Desarrollo de una API REST con Rust y Actix Web

Necesitas construir una API REST que maneje transacciones financieras en un sistema de banca. La API debe registrar transacciones con detalles como monto, fecha, tipo de transacción (depósito o retiro) y estado (pendiente, completada, fallida). Debes asegurarte de que la API maneje correctamente los errores y valide los datos de entrada. La API se integrará con un sistema de base de datos para persistir las transacciones.

## Informacion General

| Campo | Valor |
|-------|-------|
| **Tema** | Rust Actix Web |
| **Nivel** | junior-l1 |
| **Tipo** | practical |
| **Tiempo estimado** | 8 horas |

## Fases del Reto

### Fase 0: Configuración del Proyecto

**Objetivo:** Obtener el proyecto base funcional enviando el Código Base a un asistente de IA, que lo analizará, corregirá errores y generará un ZIP listo para usar.

**Tiempo estimado:** 15-30 minutos

**Instrucciones:**

- Asegúrate de tener instalado para ejecutar el proyecto: Un IDE o editor de código.
- Copia todo el contenido del campo **Código Base** de este reto — incluyendo el texto de instrucciones que aparece al inicio.
- Abre un asistente de IA (Claude en claude.ai, ChatGPT o Gemini — se recomienda Claude), pega el contenido copiado en el chat y envíalo.
- El asistente analizará los archivos, corregirá errores y generará un archivo ZIP descargable. Descárgalo y extráelo en la carpeta donde quieras trabajar.
- Verifica que el proyecto arranca sin errores.

**Entregable:** El proyecto compila/arranca sin errores.

<details>
<summary>Pistas de conocimiento</summary>

- Copia el Código Base completo incluyendo el texto de instrucciones al inicio — esas instrucciones le indican al asistente exactamente qué hacer con los archivos.
- Si el asistente no genera el ZIP automáticamente al terminar el análisis, escríbele: "genera el ZIP ahora".
- Si el proyecto tiene errores al arrancar, comparte el mensaje de error con el mismo asistente para que lo corrija.

</details>

### Fase 1: Definición y registro de transacciones

**Objetivo:** Crear una API que permita registrar transacciones financieras con validación de datos.

**Tiempo estimado:** 3 horas

**Instrucciones:**

- Define los campos necesarios para una transacción (monto, fecha, tipo, estado).
- Implementa la validación de datos para asegurar que los montos sean positivos y los tipos de transacción sean válidos.
- Crea un endpoint para registrar una nueva transacción.

**Entregable:** Endpoint funcional para registrar transacciones con validación de datos.

<details>
<summary>Pistas de conocimiento</summary>

- Considera cómo manejar errores de validación y cómo comunicarlos al usuario.
- Piensa en la estructura de datos adecuada para representar una transacción.

</details>

### Fase 2: Persistencia de transacciones

**Objetivo:** Integrar la API con una base de datos para persistir las transacciones.

**Tiempo estimado:** 3 horas

**Instrucciones:**

- Elige una estrategia para persistir las transacciones en la base de datos.
- Implementa la lógica necesaria para guardar una transacción en la base de datos.
- Asegura que la API maneje correctamente los errores de persistencia.

**Entregable:** Endpoint que registra transacciones y las persiste en la base de datos.

<details>
<summary>Pistas de conocimiento</summary>

- Considera las ventajas y desventajas de diferentes estrategias de persistencia.
- Piensa en cómo manejar los errores de conexión a la base de datos.

</details>

### Fase 3: Consulta de transacciones

**Objetivo:** Crear endpoints para consultar transacciones por diferentes criterios.

**Tiempo estimado:** 2 horas

**Instrucciones:**

- Implementa un endpoint para consultar todas las transacciones.
- Implementa un endpoint para consultar transacciones por tipo (depósito o retiro).
- Implementa un endpoint para consultar transacciones por estado (pendiente, completada, fallida).

**Entregable:** Endpoints funcionales para consultar transacciones por diferentes criterios.

<details>
<summary>Pistas de conocimiento</summary>

- Considera cómo optimizar las consultas para manejar un gran volumen de datos.
- Piensa en cómo estructurar la respuesta para que sea fácil de entender y usar.

</details>

## Dimensiones Evaluadas

- **queEs**: ¿Qué es una transacción financiera y cuáles son sus componentes?
- **paraQueSirve**: ¿Para qué sirve validar los datos de entrada en una API REST?
- **comoSeUsa**: ¿Cómo se usa un endpoint para registrar una transacción?
- **erroresComunes**: ¿Cuáles son los errores comunes al persistir datos en una base de datos y cómo se manejan?
- **queDecisionesImplica**: ¿Qué decisiones implica elegir una estrategia de persistencia para una API REST?

## Criterios de Evaluacion

- Implementar un endpoint para registrar transacciones con validación de datos.
- Integrar la API con una base de datos para persistir las transacciones.
- Crear endpoints para consultar transacciones por diferentes criterios.

---

*Reto generado automaticamente por Challenge Generator - Pragma*
