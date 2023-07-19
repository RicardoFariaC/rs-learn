static _Y:u32 = 13;

fn main() {
    // memoria static:
    /*
        utilizada em variáveis static;
        tamanho fixo;
        tempo de vida durante a execução do programa;
        cleanup após o fim do programa;
        
     */

    // memoria stack:
    /*
        utilizada em tipos primitivos não static;
        tamanho dinâmico;
        tempo de vida durante uma função;
        cleanup após o retorno da função;
        cada thread tem uma stack isolada;
    
     */

    // memoria heap:
    /*
        tamanho dinâmico até o limite do computador;
        tamanho de vida definido pelo programador ou programa;
        cleanup após deleção manual, via Garbage Collector, via RAII;
        compartilhada através de threads;
        grandes valores;

     */

    // limpeza de memória:
    /*
        utilização de RAII;
        drop para limpar objetos head;
        

     */


}
