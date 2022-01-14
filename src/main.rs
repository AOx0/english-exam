use std::collections::HashSet;
use rand::Rng;
use std::io::Write;

#[derive(Default)]
struct Verbo {
    pub presente: &'static str,
    pub p_simple: &'static str,
    pub p_participio: &'static str,
    pub significado:  &'static [&'static str]
}

macro_rules! v {
    ($p: expr, $s: expr, $pp: expr, $ss: expr) => {
        Verbo {
            presente: $p,
            p_simple: $s,
            p_participio: $pp,
            significado: $ss
        }
    };
}


const BANCO: [Verbo; 52] = [
    v!("be", "was/were", "been", &["ser"]),
    v!("become", "became", "become", &["convertirse"]),
    v!("begin", "began", "begun", &["comenzar", "empezar"]),
    v!("break", "broke", "broken", &["romper"]),
    v!("bring", "brought", "brought", &["traer"]),
    v!("build", "built", "built", &["construir"]),
    v!("buy", "bought", "bought", &["comprar"]),
    v!("can", "could", "-", &["poder"]),
    v!("catch", "caught", "caught", &["capturar", "cachar"]),
    v!("come", "came", "come", &["venir"]),
    v!("cost", "cost", "cost", &["costar"]),
    v!("do", "did", "done", &["hacer"]),
    v!("drink", "drank", "drunk", &["beber"]),
    v!("eat", "ate", "eaten", &["comer"]),
    v!("fall", "fell", "fallen", &["caer"]),
    v!("feel", "felt", "felt", &["sentir"]),
    v!("find", "found", "found", &["encontrar"]),
    v!("fly", "flew", "flown", &["volar"]),
    v!("forget", "forgot", "forgotten", &["olvidar"]),
    v!("get", "got", "got", &["obtener"]),
    v!("give", "gave", "given", &["dar"]),
    v!("go", "went", "gone", &["ir"]),
    v!("have", "had", "had", &["tener"]),
    v!("hear", "heard", "heard", &["escuchar", "oír"]),
    v!("know", "knew", "known", &["saber", "conocer"]),
    v!("leave", "left", "left", &["dejar"]),
    v!("lose", "lost", "lost", &["perder"]),
    v!("make", "made", "made", &["hacer"]),
    v!("meet", "met", "met", &["conocer"]),
    v!("pay", "paid", "paid", &["pagar"]),
    v!("put", "put", "put", &["poner"]),
    v!("read", "read", "read", &["leer"]),
    v!("run", "ran", "run", &["correr"]),
    v!("say", "said", "said", &["decir"]),
    v!("see", "saw", "seen", &["ver"]),
    v!("send", "sent", "sent", &["enviar"]),
    v!("sing", "sang", "sung", &["cantar"]),
    v!("sit", "sat", "sat", &["sentarse"]),
    v!("sleep", "slept", "slept", &["dormir"]),
    v!("speak", "spoke", "spoken", &["hablar"]),
    v!("spend", "spent", "spent", &["gastar"]),
    v!("stand", "stood", "stood", &["estar", "mantenerse"]),
    v!("swim", "swam", "swum", &["nadar"]),
    v!("teach", "taught", "taught", &["enseñar"]),
    v!("take", "took", "taken", &["llevar"]),
    v!("tell", "told", "told", &["contar", "decir"]),
    v!("think", "thought", "thought", &["pensar"]),
    v!("understand", "understood", "understood", &["comprender", "entender"]),
    v!("wake", "woke", "woken", &["despertarse", "despertar", "levantarse"]),
    v!("wear", "wore", "worn", &["vestir"]),
    v!("win", "won", "won", &["ganar"]),
    v!("write", "wrote", "written", &["escribir"])
];

macro_rules! printf {
    ( $($t:tt)* ) => {
        {
            let mut h = std::io::stdout();
            write!(h, $($t)* ).unwrap();
            h.flush().unwrap();
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut wrong = HashSet::new();

    printf!("Numero de preguntas: ");
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let mut n: i32 = n.replace("\n", "").replace(" ", "")
        .parse().unwrap_or_else(|_| {
            const D: i32 =  52*3*2;
            printf!("Default: {}\n\n",D);
            D
        }
    );

    let mut mal = 0;

    for h in 0..n {
        let i: usize = rng.gen_range(0..BANCO.len());
        let j: usize = rng.gen_range(1..4);

        let verbo = &BANCO[i];

        match j {
            1 => printf!("{:<3}.- Pasado simple de {}: ", h+1, verbo.presente),
            2 => printf!("{:<3}.- Pasado participio de {}: ", h+1, verbo.presente),
            3 => printf!("{:<3}.- Significado de {}: ", h+1, verbo.presente),
            _ => {}
        }

        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer).unwrap();
        let answer = answer.replace("\n", "").replace(" ", "");
        if answer == "stop" {
            n = h;
            break;
        }
        let temp : String;

        if match j {
            1 => verbo.p_simple.eq(&answer),
            2 => verbo.p_participio.eq(&answer),
            3 => verbo.significado.contains(&&*answer),
            _ => {false}
        } {
            printf!("Correcto!\n");
        } else {
            printf!("Incorrecto! Es {:?}\n", match j {
                1 => verbo.p_simple,
                2 => verbo.p_participio,
                3 => {
                    temp = verbo.significado.join(" o ");
                    &temp
                },
                _ => "Error"
            });
            wrong.insert(verbo.presente);
            mal += 1;
        }

    }


    printf!("\nCorrectos: {}\n", n - mal);
    printf!("Incorrectos: {}\n", mal);

    if wrong.len() > 0 {
        printf!("Verbos a estudiar: ({})\n", wrong.len());
        wrong.into_iter().for_each(|v|
            printf!("    {v}\n")
        )
    } else {
        printf!("Sin verbos a estudiar\n");
    }

}
