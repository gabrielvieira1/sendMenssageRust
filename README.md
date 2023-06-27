# sendMessageRust

> Essas implementações em Rust a funcionalidade onde o cliente envia a mensagem "Hello from client!" para o servidor e o servidor a exibe no console. A linguagem Rust oferece garantias de segurança de memória e thread-safety em tempo de compilação, permitindo escrever código seguro e robusto, sem a necessidade de gerenciamento manual da memória. Além disso, a Rust possui uma ergonomia de programação assíncrona que facilita a escrita de código eficiente e concorrente.

Ao usar a linguagem Rust em conjunto com a biblioteca tokio para programação assíncrona, você pode desfrutar de vários benefícios em comparação com a implementação de um servidor escrito em C++. Aqui estão alguns desses benefícios:

- Segurança de memória: Rust garante a segurança de memória em tempo de compilação por meio de seu sistema de propriedade (ownership) e regras de empréstimo (borrowing). Isso evita erros comuns de segurança, como vazamentos de memória, referências inválidas ou uso após a liberação. Em contraste, em C++, você precisa lidar manualmente com a alocação e desalocação de memória, o que pode levar a erros difíceis de depurar, como vazamentos de memória ou corrupção de memória.

- Ausência de erros de segurança relacionados a ponteiros nulos: Em Rust, é impossível ter ponteiros nulos não controlados. O sistema de tipos de Rust garante que todas as referências sejam válidas, eliminando erros comuns como dereferenciar um ponteiro nulo. Em C++, ponteiros nulos podem ser uma fonte comum de bugs, resultando em crashes ou comportamento indefinido.

- Programação assíncrona robusta: Rust e a biblioteca tokio fornecem uma abordagem poderosa para programação assíncrona. O modelo de programação assíncrona de Rust permite que você lide eficientemente com milhares de conexões simultâneas, sem a necessidade de thread pooling ou criação manual de threads. Através do mecanismo de tarefas assíncronas e do conceito de await, você pode escrever código conciso e eficiente que aproveita ao máximo os recursos do sistema. Em C++, a programação assíncrona geralmente é mais complexa, exigindo o uso de bibliotecas externas ou chamadas de sistema específicas.

- Thread-safety em tempo de compilação: Rust fornece garantias de thread-safety em tempo de compilação por meio de seu sistema de propriedade e regras de empréstimo. Isso permite escrever código concorrente seguro sem preocupações com race conditions, data races ou problemas relacionados à sincronização manual. Em C++, a thread-safety é deixada nas mãos do programador, exigindo o uso de primitivas de sincronização, como mutexes, semáforos ou variáveis atômicas, aumentando a complexidade e propensão a erros.

- Ecossistema forte e moderno: Rust possui um ecossistema de bibliotecas robusto e em crescimento, com muitas ferramentas e recursos disponíveis para desenvolvedores. A biblioteca tokio é um exemplo disso, oferecendo suporte completo para programação assíncrona. Rust também possui um gerenciador de pacotes, o Cargo, que facilita a inclusão e atualização de dependências.

- Em resumo, ao usar Rust com a biblioteca tokio, você obtém segurança de memória, programação assíncrona eficiente, thread-safety em tempo de compilação e um ecossistema moderno, permitindo desenvolver aplicativos robustos e seguros com menos erros relacionados a memória, concorrência e segurança.
