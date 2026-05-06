{{-- 23 lines 15 code 5 comments 3 blanks --}}

{{--
    A welcome page demonstrating Blade syntax.
--}}
@extends('layouts.app')

@section('content')
    <!-- Page header -->
    <div class="container">
        <h1>{{ $title }}</h1>

        @if ($users->isNotEmpty())
            <ul class="list-group">
                @foreach ($users as $user)
                    <li>{{ $user->name }}</li>
                @endforeach
            </ul>
        @else
            <p>No users found.</p>
        @endif
    </div>
@endsection
