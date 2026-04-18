import { inject, Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable, BehaviorSubject, tap } from 'rxjs';
import { environment } from '../../environments/environment';
import { Router } from '@angular/router';

@Injectable({ providedIn: 'root' })
export class AuthService {
    private _loggedIn$ = new BehaviorSubject<boolean>(false);
    public isLoggedIn$ = this._loggedIn$.asObservable();

    private _http = inject(HttpClient)
    private _router = inject(Router)

    checkLogin(): Observable<any> {
        return this._http.get(`${environment.baseUrl}/api/v1/login`, { withCredentials: true }).pipe(
            tap(() => this._loggedIn$.next(true))
        )
    }

    refresh(): Observable<any> {
        return this._http.post(`${environment.baseUrl}/api/v1/refresh`, {}, { withCredentials: true }).pipe(
            tap(() => this._loggedIn$.next(true))
        )
    }

    login(username: string, password: string): Observable<any> {
        const form = new HttpParams()
            .set('username', username)
            .set('password', password)

        return this._http.post(`${environment.baseUrl}/api/v1/login`, form).pipe(
            tap(() => this._router.navigate(['/']))
        )
    }

    logout(): Observable<any> {
        const callback = () => {
            this._loggedIn$.next(false)
            this._router.navigate(['/login'])
        };
        return this._http.post(`${environment.baseUrl}/api/v1/logout`, {}, { withCredentials: true }).pipe(
            tap({ next: callback, error: callback })
        )
    }
}
