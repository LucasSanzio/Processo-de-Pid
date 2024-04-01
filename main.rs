use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct Processo {
    pid: usize,
    tamanho_memoria: usize,
    tempo_execucao: u64,
}

impl Processo {
    fn novo(pid: usize, tamanho_memoria: usize, tempo_execucao: u64) -> Self {
        Self {
            pid,
            tamanho_memoria,
            tempo_execucao,
        }
    }

    fn executar(&self) {
        println!("Iniciando execução do processo PID {}", self.pid);
        let start_time = Instant::now();
        let progresso_barra_tamanho = 20;

        let mut tempo_decorrido: u64 = 0;
        while tempo_decorrido < self.tempo_execucao {
            let progresso = ((tempo_decorrido as f64 / self.tempo_execucao as f64) * progresso_barra_tamanho as f64) as usize;
            let progresso_restante = progresso_barra_tamanho - progresso;
            print!("\r[{}{}] - Processo PID {} em execução...", "-".repeat(progresso), " ".repeat(progresso_restante), self.pid);
            thread::sleep(Duration::from_secs(1));
            tempo_decorrido = start_time.elapsed().as_secs();
        }

        print!("\r[{}{}] - Processo PID {} em execução... ", "-".repeat(progresso_barra_tamanho), "", self.pid);
        println!("Processo PID {} concluído.", self.pid);
    }
}

struct PilhaProcessos {
    processos: Vec<Processo>,
    memoria_total: usize,
    tempo_total: u64,
}

impl PilhaProcessos {
    fn nova() -> Self {
        Self {
            processos: Vec::new(),
            memoria_total: 0,
            tempo_total: 0,
        }
    }

    fn empilhar(&mut self, processo: Processo) {
        self.processos.push(processo.clone());
        self.memoria_total += processo.tamanho_memoria;
        self.tempo_total += processo.tempo_execucao;
    }

    fn executar_processo_topo(&mut self) {
        if let Some(processo) = self.processos.pop() {
            processo.executar();
        } else {
            println!("Pilha de processos vazia.");
        }
    }

    fn mostrar_resumo(&self) {
        println!("Resumo dos processos na pilha:");
        for processo in &self.processos {
            println!("PID: {}", processo.pid);
        }
    }

    fn mostrar_resumo_execucao(&self) {
        println!("Resumo da execução dos processos:");
        for processo in &self.processos {
            println!("Processo PID {} executado.", processo.pid);
        }
        println!("Consumo total de memória: {} bytes", self.memoria_total);
        println!("Tempo total gasto na execução: {} segundos", self.tempo_total);
    }
}

fn main() {
    let mut pilha_processos = PilhaProcessos::nova();

    for pid in 0..3 {
        println!("Digite os dados para o processo {}:", pid);
        println!("Tamanho de memória utilizada (em bytes):");
        let tamanho_memoria: usize = ler_entrada_usuario();
        println!("Tempo de execução (entre 30 e 90 segundos):");
        let tempo_execucao: u64 = ler_entrada_usuario();

        if tempo_execucao < 30 || tempo_execucao > 90 {
            println!("Tempo de execução fora do intervalo permitido. Utilizando 30 segundos por padrão.");
            pilha_processos.empilhar(Processo::novo(pid, tamanho_memoria, 30));
        } else {
            pilha_processos.empilhar(Processo::novo(pid, tamanho_memoria, tempo_execucao));
        }
    }
    pilha_processos.mostrar_resumo();

    while !pilha_processos.processos.is_empty() {
        pilha_processos.executar_processo_topo();
    }

    pilha_processos.mostrar_resumo_execucao();
}

fn ler_entrada_usuario<T>() -> T
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
{
    use std::io::{self, Write};
    let mut input = String::new();
    io::stdout().flush().expect("Falha ao limpar stdout");
    io::stdin().read_line(&mut input).expect("Falha ao ler stdin");
    input.trim().parse().expect("Falha ao analisar entrada")
}
