<img src="../../logo.png" heigth=300>

---

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/rustkindergarten)

[**MENU do curso**](../../README.md)

# Devê di casa - Exercícios para a mente e o corpo

> Esta aula não tem vídeo.

Para começar, [**já instalou o Rust?**](https://www.mycompiler.io/pt/new/rust).

Faça todos usando o `cargo`! Não quero mais ver `rustc` viu? 

Aqui estão **10 exercícios fáceis em Rust** para quem está começando, cada um focando em um conceito básico:

1. **Verificar par ou ímpar**
   Leia um número inteiro da linha de comando e diga se ele é par ou ímpar. (resto de divisão é com o operador `%`).
   Para ler um argumento e transformar em número, use: `let resto = args[1].parse::<u32>().unwrap()`. O `unwrap()` desembrula o `Result<>` do `parse`. 

2. **Reverter string**
   Leia uma palavra e mostre-a invertida (dica: use `.chars().rev().collect::<String>()`).

3. **Contar vogais**
   Conte quantas vogais existem em uma string fornecida pelo usuário.
   Dica 1: Pode navegar nos caracteres do `String` com `for ... in ....chars()`.
   Dica 2: Pode usar o `contains` para isso: `"aeiouAEIOU".contains(<variável do caractere>)`.

4. **Cálculo de média**
   Leia uma lista de notas de um arquivo e calcule a média.

5. **Implemente o crivo de Eratóstenes em Rust**

```
Início
    Leia N
    Crie um vetor PRIMO[0..N] e marque todos como VERDADEIRO
    PRIMO[0] ← FALSO
    PRIMO[1] ← FALSO

    Para p de 2 até √N faça
        Se PRIMO[p] = VERDADEIRO então
            Para múltiplo de p, começando em p*p até N, passo p faça
                // Em rust: for j in (i*i..limite+1).step_by(i as usize) 
                PRIMO[múltiplo] ← FALSO
            FimPara
        FimSe
    FimPara

    Escreva "Números primos até N:"
    Para i de 2 até N faça
        Se PRIMO[i] = VERDADEIRO então
            Escreva i
        FimSe
    FimPara
Fim
```

Crie uma função que retorne um vetor de primos até o número dado.  

6. Crie um programa que use a função do exercício anterior e fatore um número dado. 





