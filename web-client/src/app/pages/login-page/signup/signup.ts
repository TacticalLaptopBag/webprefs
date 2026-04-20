import { Component, inject, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AuthService } from '../../../services/auth.service';

@Component({
    selector: 'app-signup',
    imports: [FormsModule],
    templateUrl: './signup.html',
    styleUrl: './signup.css',
})
export class Signup {
    public errorMsg = signal<string | null>(null)
    public username = signal('')
    public password = signal('')
    public passwordConfirm = signal('')
    public isLoading = signal(false)

    private _authSvc = inject(AuthService)

    public onSubmit() {
        if (this.password() !== this.passwordConfirm()) {
            this.errorMsg.set('Passwords do not match')
            return
        }

        this.isLoading.set(true)
        this.errorMsg.set(null)
        this._authSvc.signup(this.username(), this.password()).subscribe({
            error: (err) => {
                const errMsg = err?.message ?? err?.error?.error ?? err?.error ?? err
                this.errorMsg.set(errMsg?.toString())
                this.isLoading.set(false)
            }
        })
    }
}
