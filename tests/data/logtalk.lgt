/*
	Test file for the Logtalk programming language
	(copied by the author from a Logtalk distribution example)

	65 lines 27 code 18 comments 20 blanks
*/


% Alf believes he is the only survivor of his species; no point in
% defining a class if there is only going to be a single instance:

% a prototype, which is also a stand-alone object

:- object(alf).

	% prototypes declare predicates for themselves (and derived prototypes)
	:- public([
		name/1, planet/1, stomachs/1, favorite_food/1, chases/1, motto/1
	]).

	name('Gordon Shumway').
	planet('Melmac').
	stomachs(8).
	favorite_food(cats).
	chases('Lucky').
	motto('Are you going to finish that sandwich?').

:- end_object.


% later on, Alf finds out that his best friend, Skip, and his
% girlfriend, Rhonda, also survived Melmac's explosion; as they
% are all melmacians, they share most attributes (and add some
% of their own):

% "skip", a derived prototype from "alf", its parent prototype

:- object(skip,
	extends(alf)).

	:- public(best_friend/1).

	best_friend(alf).
	name('Skip').
	% still longing for a nice cat to eat since Melmac exploded
	chases(_) :-
		fail.

:- end_object.


% "rhonda" is also a prototype derived from "alf"

:- object(rhonda,
	extends(alf)).

	:- public(boyfriend/1).

	boyfriend(alf).
	name('Rhonda').
	% still longing for a nice cat to eat since Melmac exploded
	chases(_) :-
		fail.

:- end_object.
