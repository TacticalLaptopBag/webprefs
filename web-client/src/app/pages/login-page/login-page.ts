import { Component, inject, OnInit, signal } from '@angular/core';
import { Login } from "./login/login";
import { Router } from '@angular/router';
import { AuthService } from '../../services/auth.service';
import { take } from 'rxjs';
import { environment } from '../../../environments/environment';
import { Signup } from "./signup/signup";

@Component({
    selector: 'app-login-page',
    imports: [Login, Signup],
    templateUrl: './login-page.html',
    styleUrl: './login-page.css',
})
export class LoginPage implements OnInit {
    public currentTab = signal('login')
    public signupEnabled = environment.signupEnabled

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
