/* Talking with a contract often involves transforming data, we recommend you to encapsulate that logic into a class */
export class boardChessBoard {
  constructor({ contractId, walletToUse }) {
    this.contractId = contractId;
    this.wallet = walletToUse;    
  }

  async get_fen() {
    return await this.wallet.viewMethod(
      { 
        contractId: this.contractId, 
        method: 'get_fen' 
      }
    );
  }

  async add_player() {
    return await this.wallet.callMethod(
      { 
        contractId: this.contractId, 
        method: 'add_player', 
        args: { 
          player_address: this.wallet.accountId 
        } 
      }
    );
  }

  async cast_vote(fen) {
    return await this.wallet.callMethod(
      { 
        contractId: this.contractId, 
        method: 'add_player', 
        args: { 
          board_fen: fen 
        } 
      }
    );
  }

  async tally_votes() {
    return await this.wallet.callMethod(
      { 
        contractId: this.contractId, 
        method: 'tally_votes', 
      }
    );
  }
}