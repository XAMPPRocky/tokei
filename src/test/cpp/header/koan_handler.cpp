/*
    Copyright (c) 2012 Torbjörn Klatt <opensource@torbjoern-klatt.de>

    Permission is hereby granted, free of charge, to any person
    obtaining a copy of this software and associated documentation
    files (the "Software"), to deal in the Software without
    restriction, including without limitation the rights to use,
    copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following
    conditions:

    The above copyright notice and this permission notice shall be
    included in all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
    OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
    NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
    HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
    WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
    OTHER DEALINGS IN THE SOFTWARE.
*/


#include "koan_handler.hpp"

using namespace std;

KoanHandler::KoanHandler() : total_num_koans( 0 ), total_num_passed( 0 )
{
}

void KoanHandler::eval_koan( Koan obj, void ( Koan::*koan )() )
{
  try {
    ( obj.*koan )();
    this->total_num_passed++;
  } catch( FillMeInException ex ) {
    this->print_failure( ex );
    exit( 1 );
  }
}

void KoanHandler::register_koans( int num_koans )
{
  this->total_num_koans += num_koans;
}

void KoanHandler::start()
{
  cout << "*******************************************************************************"
       << endl
       << "****                          Welcome to CppKoans                          ****"
       << endl
       << "****                     The master has " << this->total_num_koans
       << " koans for you.                  ****"
       << endl << endl;
}

void KoanHandler::end()
{
  cout << "****    You've walked the path to enlightment and satisfied your master.   ****"
       << endl
       << "*******************************************************************************"
       << endl;
}

void KoanHandler::episode_start( string order )
{
  cout << "****\tThe " << order << " episode." << endl;
}

void KoanHandler::episode_done( string order )
{
  cout << "****\tCongratulations! You mastered the " << order << " episode." << endl
       << "****\tDon't loose concentration. Keep going!" << endl
       << endl;
}

void KoanHandler::print_failure( FillMeInException ex )
{
  if( !ex.msg.empty() ) {
    cout << "Note:\t" << ex.msg << endl;
  }
  cout << "The master says, that you should meditate on '" << ex.file << ":"
       << ex.line << "'."
       << endl;
  if( !ex.expect.empty() ) {
    cout << "The master expected " << ex.expect << "." << endl;
  }
  cout << endl
       << "****                      You mastered " << this->total_num_passed << " of "
       << this->total_num_koans << " koans.                      ****"
       << endl
       << "****                              Keep going!                              ****"
       << endl;
}

// EOF
