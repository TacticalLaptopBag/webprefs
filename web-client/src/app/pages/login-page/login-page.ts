import { Component, inject, OnInit } from '@angular/core';
import { Login } from "./login/login";
import { Router } from '@angular/router';
import { AuthService } from '../../services/auth.service';
import { take } from 'rxjs';

@Component({
    selector: 'app-login-page',
    imports: [Login],
    templateUrl: './login-page.html',
    styleUrl: './login-page.css',
})
export class LoginPage implements OnInit {
    private _router = inject(Router)
    private _authService = inject(AuthService)

    ngOnInit(): void {
        this._authService.isLoggedIn$.pipe(take(1)).subscribe((isLoggedIn: boolean) => {
            if (isLoggedIn) {
                this._router.navigate(['/'])
            }
        })
    }
}
