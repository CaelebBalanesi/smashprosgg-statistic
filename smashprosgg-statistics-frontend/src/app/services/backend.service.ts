import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Injectable({
  providedIn: 'root'
})
export class BackendService {

  constructor(
    private http: HttpClient,
  ) {

  }

  username!: string;

  get_set_winrate(username: string) {
    console.log("http://localhost:2222/game_winrate/user?username=" + username);
    return this.http.get<any>("http://localhost:2222/game_winrate/user?username=" + username);
  }

  get_game_winrate() {

  }

  set_username(username: string) {
    this.username = username;
  }
}
