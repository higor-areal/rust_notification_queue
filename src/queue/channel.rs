use tokio::sync::mpsc;
use crate::models::notification_job::NotificationJob;

pub fn create_channel() -> (mpsc::Sender<NotificationJob>, mpsc::Receiver<NotificationJob>) {
    mpsc::channel(100)
}

//aqui nos temos algo interessante do mpsc do tokio, ele cria um channel que tem duas chaves um tx = Sender = enviador e rx = Receiver = recebedor, e de alguma maneira muito interessante estamos padronizando, colocando que vamos envia um NotificationJob e receber o mesmo, e creio eu que possamos só enviar e receber esse mesmo dado.
//creio eu que essa função de criar channel ou cria sem tipo, podendendo enviar qualquer coisas, ou como nos dizemos o tipo de retorno ela infere sozinha

//acabei de pergutar do chat se era isso mesmo e ele me disse que o channel não pode existir sem um tipo, posso dizer que é um box dyn mas estou sendo explicito aí, e nessa nossa função o rust consegue inferir o tipo através da nossa assinatura 