import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class BackendService {

  constructor() {

  }

  username!: string;

  get_set_winrate() {

  }

  get_game_winrate() {

  }

  set_username(username: string) {
    this.username = username;
  }
}
