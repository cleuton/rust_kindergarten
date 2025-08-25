<img src="logo.png" heigth=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/rustkindergarten)

[**MENU do curso**](README.md)

# Instalar o Rust

Use o script [**rustup**](https://www.rust-lang.org/tools/install) para instalar o `toolchain` do **Rust**, que inclui o `cargo`. 

Para isso temos que considerar o seu sistema operacional.

## MacOS ou Linux

Simplesmente baixe e execute o `rustup`: 

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Este script instala todo o `toolchain` dentro da sua pasta `home/.cargo/bin`. Ele tentará ajustar a variável `PATH` Para incluir essa pasta. Após a instalação, tente usar o comando `rustc --version` se ele não for encontrado, é só adicionar a pasta `~/.cargo/bin` à variável `PATH`, o que pode ser feito no seu script de login. 

## MS Windows

#### Ambiente C++

Para que tudo funcione sem surpresas você poderá precisar de um compilador C++! se o seu projeto for usar `crates` (pacotes Rust) com código nativo (por exemplo, C/C++), o rustc vai chamar um linker externo e só o encontrará se você já tiver instalado aquele ambiente de compilação. Na prática, a ordem é:

- Instale primeiro o Visual C++ Build Tools (desktop C++) ou o MinGW-w64, garantindo que cl.exe ou gcc/ld estejam no seu `PATH`. 

Entendeu? 

#### Chocolatey

Se você tiver um instalador de pacotes como o [**Chocolatey**](http://chocolatey.org/) a instalação é simples: 

1) Abra um CMD como `Administrator`;
2) Digite o comando: `choco install rust -y`;
3) Feche o CMD e reabra (não precisa ser administrador);
4) Execute: `cargo --version`.

#### Usando rustup-init.exe

Para instalar o Rust usando o instalador oficial, faça o seguinte em um prompt de comando no Windows:

Abra o navegador e acesse [https://rustup.rs](https://rustup.rs). Clique em “Download rustup-init.exe” e salve o arquivo em uma pasta de fácil acesso. Em seguida, abra o Prompt de Comando, navegue até essa pasta (por exemplo, `cd C:\Users\SeuUsuário\Downloads`) e execute:

```bat
rustup-init.exe
```

O instalador vai mostrar algumas opções — basta pressionar Enter para optar pelo canal “stable” e aceitar a configuração padrão (que já inclui o `rustc`, o `cargo` e as ferramentas auxiliares). Quando tudo for instalado, feche aquele terminal e abra um novo. Digite:

```bat
rustc --version
cargo --version
```

Se aparecerem as versões do Rust e do Cargo, a instalação foi bem-sucedida. A partir daí, você já pode criar projetos com `cargo new` e compilar normalmente.
